use crate::assembler::Assembler;
use crate::assembler::state::State;
use crate::pose::Pose;

pub struct Executor {
    pose: Pose,
    pub(crate) state: Box<dyn Assembler>,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
            state: Box::new(State::default()),
        }
    }

    pub(crate) fn with_pose_and_state(pose: Pose, state: Box<dyn Assembler>) -> Self {
        Executor { pose, state }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B' => self.state.be_reverse(),
                'F' => self.state.be_fast(),
                _ => {
                    let actions = self.state.assemble(cmd);
                    for action in actions {
                        action.perform(&mut self.pose);
                    }
                }
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
