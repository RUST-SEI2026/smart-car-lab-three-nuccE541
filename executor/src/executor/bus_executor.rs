use crate::Pose;

pub struct Executor {
    pose: Pose,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        todo!()
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
