// Copyright (C) Microsoft Corporation. All rights reserved.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use util::{csprng::Csprng, prime::BigUintPrime};

use crate::{
    contest_hash,
    contest_selection::ContestSelection,
    device::Device,
    election_manifest::{Contest, ContestIndex, ContestOptionIndex},
    election_record::PreVotingData,
    fixed_parameters::FixedParameters,
    hash::HValue,
    index::Index,
    joint_election_public_key::{Ciphertext, Nonce},
    nonce::encrypted as nonce,
    vec1::Vec1,
    zk::ProofRange,
};

// /// A contest.
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Contest {
//     /// The label.
//     pub label: String,

//     /// The maximum count of options that a voter may select.
//     pub selection_limit: usize,

//     /// The candidates/options. The order of options matches the virtual ballot.
//     pub options: Vec<ContestOption>,
// }

// /// An option in a contest.
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct ContestOption {
//     /// The label.
//     pub label: String,
// }

/// A 1-based index of a [`ContestEncrypted`] in the order it is defined in the [`crate::ballot::BallotEncrypted`].
pub type ContestEncryptedIndex = Index<ContestEncrypted>;

/// A contest in an encrypted ballot.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContestEncrypted {
    /// Encrypted voter selection vector.
    pub selection: Vec<Ciphertext>,

    /// Contest hash.
    pub contest_hash: HValue,

    /// Proof of ballot correctness.
    pub proof_ballot_correctness: Vec1<ProofRange>,

    // Proof of satisfying the selection limit.
    pub proof_selection_limit: ProofRange,
}

/// A scaled version of [`ContestEncrypted`]. This means that each encrypted vote on the contest
/// has been scaled by a factor. It is trusted that the encrypted ciphertexts in a
/// [`ScaledContestEncrypted`] really are the ones from a [`ContestEncrypted`] scaled by a factor.
/// Contains no proofs.
pub struct ScaledContestEncrypted {
    /// Scaled encrypted voter selection vector.
    pub selection: Vec<Ciphertext>,
}

impl ContestEncrypted {
    fn encrypt_selection(
        header: &PreVotingData,
        primary_nonce: &[u8],
        contest_index: ContestIndex,
        pt_vote: &ContestSelection,
    ) -> Vec<(Ciphertext, Nonce)> {
        // TODO: Check if selection limit is satisfied

        let mut vote: Vec<(Ciphertext, Nonce)> = Vec::new();
        for j in 1..pt_vote.vote.len() + 1 {
            #[allow(clippy::unwrap_used)] //? TODO: Remove temp development code
            let o_idx = ContestOptionIndex::from_one_based_index(j as u32).unwrap();
            let nonce = nonce(header, primary_nonce, contest_index, o_idx);
            vote.push((
                header.public_key.encrypt_with(
                    &header.parameters.fixed_parameters,
                    &nonce,
                    pt_vote.vote[j - 1] as usize,
                ),
                Nonce::new(nonce),
            ));
        }
        vote
    }

    pub fn new(
        device: &Device,
        csprng: &mut Csprng,
        primary_nonce: &[u8],
        contest: &Contest,
        contest_index: ContestIndex,
        pt_vote: &ContestSelection,
    ) -> ContestEncrypted {
        let selection_and_nonce =
            Self::encrypt_selection(&device.header, primary_nonce, contest_index, pt_vote);
        let selection = selection_and_nonce
            .iter()
            .map(|(ct, _)| ct.clone())
            .collect::<Vec<_>>();
        let contest_hash = contest_hash::contest_hash(&device.header, contest_index, &selection);

        let mut proof_ballot_correctness = Vec1::new();
        for (i, (sel, nonce)) in selection_and_nonce.iter().enumerate() {
            #[allow(clippy::unwrap_used)] //? TODO: Remove temp development code
            proof_ballot_correctness
                .try_push(sel.proof_ballot_correctness(
                    &device.header,
                    csprng,
                    pt_vote.vote[i] == 1u8,
                    nonce,
                    &device.header.parameters.fixed_parameters.q,
                ))
                .unwrap();
        }

        let mut num_selections = 0;
        pt_vote.vote.iter().for_each(|v| num_selections += v);

        let proof_selection_limit = ContestEncrypted::proof_selection_limit(
            &device.header,
            csprng,
            &device.header.parameters.fixed_parameters.q,
            &selection_and_nonce,
            num_selections as usize,
            contest.selection_limit,
        );

        ContestEncrypted {
            selection,
            contest_hash,
            proof_ballot_correctness,
            proof_selection_limit,
        }
    }

    pub fn get_proof_ballot_correctness(&self) -> &Vec1<ProofRange> {
        &self.proof_ballot_correctness
    }

    pub fn get_proof_selection_limit(&self) -> &ProofRange {
        &self.proof_selection_limit
    }

    pub fn proof_selection_limit(
        header: &PreVotingData,
        csprng: &mut Csprng,
        q: &BigUintPrime,
        selection: &[(Ciphertext, Nonce)],
        num_selections: usize,
        selection_limit: usize,
    ) -> ProofRange {
        let (combined_ct, combined_nonce) =
            Self::sum_selection_nonce_vector(&header.parameters.fixed_parameters, selection);
        ProofRange::new(
            header,
            csprng,
            q,
            &combined_ct,
            &combined_nonce,
            num_selections,
            selection_limit,
        )
    }

    /// Verify the proof that the selection limit is satisfied.
    fn verify_selection_limit(&self, header: &PreVotingData, selection_limit: usize) -> bool {
        let combined_ct =
            Self::sum_selection_vector(&header.parameters.fixed_parameters, &self.selection);
        ProofRange::verify(
            &self.proof_selection_limit,
            header,
            &combined_ct,
            selection_limit,
        )
    }

    pub fn sum_selection_nonce_vector(
        fixed_parameters: &FixedParameters,
        selection_with_nonces: &[(Ciphertext, Nonce)],
    ) -> (Ciphertext, Nonce) {
        // First element in the selection

        // let mut sum_ct = selection[0].clone();
        let mut sum_ct = selection_with_nonces[0].0.clone();

        let mut sum_nonce = selection_with_nonces[0].1.clone();

        // Subsequent elements in the selection

        for (sel, nonce) in selection_with_nonces.iter().skip(1) {
            sum_ct.alpha = (&sum_ct.alpha * &sel.alpha) % fixed_parameters.p.as_ref();
            sum_ct.beta = (&sum_ct.beta * &sel.beta) % fixed_parameters.p.as_ref();

            sum_nonce.xi = (&sum_nonce.xi + &nonce.xi) % fixed_parameters.q.as_ref();
        }

        (sum_ct, sum_nonce)
    }

    pub fn sum_selection_vector(
        fixed_parameters: &FixedParameters,
        selection: &[Ciphertext],
    ) -> Ciphertext {
        // First element in the selection

        // let mut sum_ct = selection[0].clone();
        let mut sum_ct = selection[0].clone();

        // Subsequent elements in the selection

        for sel in selection.iter().skip(1) {
            sum_ct.alpha = (&sum_ct.alpha * &sel.alpha) % fixed_parameters.p.as_ref();
            sum_ct.beta = (&sum_ct.beta * &sel.beta) % fixed_parameters.p.as_ref();
        }

        sum_ct
    }

    /// Verify the proof that each encrypted vote is an encryption of 0 or 1,
    /// and that the selection limit is satisfied.
    pub fn verify(&self, header: &PreVotingData, selection_limit: usize) -> bool {
        for (ct, j) in self.selection.iter().zip(1..) {
            let Ok(idx) = Index::from_one_based_index(j) else {
                return false;
            };
            let Some(proof) = self.proof_ballot_correctness.get(idx) else {
                return false;
            };
            if !ct.verify_ballot_correctness(header, proof) {
                return false;
            }
        }

        self.verify_selection_limit(header, selection_limit)
    }

    /// Scales all the encrypted votes on the contest by the same factor.
    pub fn scale(&self, fixed_parameters: &FixedParameters, factor: BigUint) -> ScaledContestEncrypted {
        let selection = self.selection.iter().map(|ct| ct.scale(fixed_parameters, factor.clone())).collect();
        ScaledContestEncrypted{selection}
    }
}
