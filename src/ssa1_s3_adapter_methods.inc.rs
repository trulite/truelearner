
        pub(super) fn offer_masked(&mut self, present: [bool; 2]) -> [usize; 2] {
            frozen_learning::s3_offer_masked(&mut self.session, present)
        }

        pub(super) fn recur_live_before_event(&mut self, route: usize) -> usize {
            frozen_learning::s3_recur_live_before_event(&mut self.session, route)
        }

        pub(super) fn state_exact(&self, other: &Self) -> bool {
            frozen_learning::s3_session_exact(&self.session, &other.session)
        }
