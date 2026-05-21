use crate::assembler::bus_state::BusState;
use crate::pose::Pose;
use super::executor::Executor;

pub struct BusExecutor;

impl BusExecutor {
    pub fn with_pose(pose: Pose) -> Executor {
        Executor::with_pose_and_state(pose, Box::new(BusState::default()))
    }
}
