use crate::assembler::sports_car_state::SportsCarState;
use crate::pose::Pose;
use super::executor::Executor;

pub struct SportsCarExecutor;

impl SportsCarExecutor {
    pub fn with_pose(pose: Pose) -> Executor {
        Executor::with_pose_and_state(pose, Box::new(SportsCarState::default()))
    }
}
