use crate::assembler::state::State;
use crate::Pose;
use crate::assembler::assembler::Assembler;

pub struct Executor {
    pose: Pose,
    state: Box<dyn Assembler>,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
            state: Box::new(State::default())
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B' => self.state.be_reverse(),
                'F' => self.state.be_fast(),
                _ => {
                    let astions = self.state.assemble(cmd);
                    for action in astions {
                        action.perform(&mut self.pose)
                    }
                }
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
