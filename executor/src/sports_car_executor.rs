use crate::Pose;

pub struct SportsCarExecutor {
    pose:Pose
}

impl SportsCarExecutor{
    pub fn with_pose(pose: Pose) -> Self {
        SportsCarExecutor {
            pose,
        }
    }

    pub fn execute(&mut self, cmds:&str) {
        todo!()
    }

    pub fn query(&self) ->Pose {
        self.pose
    }
}