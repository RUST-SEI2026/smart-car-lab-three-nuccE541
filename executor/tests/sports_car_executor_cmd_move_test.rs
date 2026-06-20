use executor::{Pose, SportsCarExecutor as Executor};
#[test]
fn should_return_x_plus_2_given_command_is_m_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("M");

    // then
    let expected_pose = Pose::new(2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_2_given_command_is_bm_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("BM");

    // then
    let expected_pose = Pose::new(-2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_4_given_command_is_fm_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FM");

    // then
    let expected_pose = Pose::new(4, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_4_given_command_is_fbm_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FBM");

    // then
    let expected_pose = Pose::new(-4, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}
