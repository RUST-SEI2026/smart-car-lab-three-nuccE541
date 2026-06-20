use crate::Pose;

pub struct  BusExecutor {
    pose: Pose,
}

impl BusExecutor {
    pub fn with_pose(pose: Pose) -> Self {
        BusExecutor {
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
