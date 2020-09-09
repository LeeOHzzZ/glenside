#[cfg(test)]
mod tests {
    use crate::language::{Language, MyAnalysis};
    use egg::{EGraph, Pattern, RecExpr, Searcher};
    use serde_json::{from_str, Value};
    use std::collections::HashMap;
    use std::io::Write;
    use std::process::Command;
    use std::str::FromStr;

    /// Creates a Relay-to-Glenside test
    /// $test_name: the name of the created test
    /// $relay_str: A string containing the Relay program to be converted
    /// $glenside_str: A string containing the expected resulting Glenside
    ///     expression
    macro_rules! test {
        ($test_name:ident, $relay_str:expr, $glenside_str:expr) => {
            #[test]
            fn $test_name() {
                let script_filepath = format!(
                    "{}/src/language/from_relay/from_relay.py",
                    env!("CARGO_MANIFEST_DIR")
                );
                // https://www.reddit.com/r/rust/comments/38jhva/piping_string_to_child_process_stdin/crvlqcd/?utm_source=reddit&utm_medium=web2x&context=3
                let mut proc = Command::new("python3")
                    .arg(script_filepath)
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                    .ok()
                    .expect("Failed to spawn process");
                proc.stdin
                    .as_mut()
                    .unwrap()
                    .write_all($relay_str.as_bytes())
                    .unwrap();
                let output = proc.wait_with_output().unwrap();
                // Check that it ran.
                assert!(
                    output.status.success(),
                    "Failed with code {:?}.\nstdout:\n{}\nstderr:\n{}",
                    output.status.code(),
                    std::str::from_utf8(output.stdout.as_slice())
                        .expect("Could not convert stderr to UTF8"),
                    std::str::from_utf8(output.stderr.as_slice())
                        .expect("Could not convert stderr to UTF8")
                );

                let json_output: Value =
                    from_str(String::from_utf8(output.stdout).unwrap().as_str()).unwrap();
                let glenside_str = json_output.get("program").unwrap().as_str().unwrap();
                let expr =
                    RecExpr::from_str(glenside_str).expect("Could not parse glenside expression");

                // Parse shape dict
                let mut shapes = HashMap::default();
                for (key, value) in json_output
                    .get("shapes")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .iter()
                {
                    shapes.insert(
                        key.clone(),
                        value
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|v| v.as_u64().unwrap() as usize)
                            .collect::<Vec<_>>(),
                    );
                }

                let mut egraph = EGraph::new(MyAnalysis {
                    name_to_shape: shapes,
                });
                let id = egraph.add_expr(&expr);

                let pattern = $glenside_str.parse::<Pattern<Language>>().unwrap();
                assert!(pattern.search_eclass(&egraph, id).is_some());
            }
        };
    }
    test!(
        mobilenet,
        r#"
#[version = "0.0.5"]
def @main(%data: Tensor[(1, 3, 224, 224), float32], %conv_block_1_conv_weight: Tensor[(32, 3, 3, 3), float32], %conv_block_1_bn_gamma: Tensor[(32), float32], %conv_block_1_bn_beta: Tensor[(32), float32], %conv_block_1_bn_moving_mean: Tensor[(32), float32], %conv_block_1_bn_moving_var: Tensor[(32), float32], %separable_conv_block_1_weight: Tensor[(32, 1, 3, 3), float32], %separable_conv_block_1_bn1_gamma: Tensor[(32), float32], %separable_conv_block_1_bn1_beta: Tensor[(32), float32], %separable_conv_block_1_bn1_moving_mean: Tensor[(32), float32], %separable_conv_block_1_bn1_moving_var: Tensor[(32), float32], %separable_conv_block_1_conv2_weight: Tensor[(64, 32, 1, 1), float32], %separable_conv_block_1_bn2_gamma: Tensor[(64), float32], %separable_conv_block_1_bn2_beta: Tensor[(64), float32], %separable_conv_block_1_bn2_moving_mean: Tensor[(64), float32], %separable_conv_block_1_bn2_moving_var: Tensor[(64), float32], %separable_conv_block_2_weight: Tensor[(64, 1, 3, 3), float32], %separable_conv_block_2_bn1_gamma: Tensor[(64), float32], %separable_conv_block_2_bn1_beta: Tensor[(64), float32], %separable_conv_block_2_bn1_moving_mean: Tensor[(64), float32], %separable_conv_block_2_bn1_moving_var: Tensor[(64), float32], %separable_conv_block_2_conv2_weight: Tensor[(128, 64, 1, 1), float32], %separable_conv_block_2_bn2_gamma: Tensor[(128), float32], %separable_conv_block_2_bn2_beta: Tensor[(128), float32], %separable_conv_block_2_bn2_moving_mean: Tensor[(128), float32], %separable_conv_block_2_bn2_moving_var: Tensor[(128), float32], %separable_conv_block_3_weight: Tensor[(128, 1, 3, 3), float32], %separable_conv_block_3_bn1_gamma: Tensor[(128), float32], %separable_conv_block_3_bn1_beta: Tensor[(128), float32], %separable_conv_block_3_bn1_moving_mean: Tensor[(128), float32], %separable_conv_block_3_bn1_moving_var: Tensor[(128), float32], %separable_conv_block_3_conv2_weight: Tensor[(128, 128, 1, 1), float32], %separable_conv_block_3_bn2_gamma: Tensor[(128), float32], %separable_conv_block_3_bn2_beta: Tensor[(128), float32], %separable_conv_block_3_bn2_moving_mean: Tensor[(128), float32], %separable_conv_block_3_bn2_moving_var: Tensor[(128), float32], %separable_conv_block_4_weight: Tensor[(128, 1, 3, 3), float32], %separable_conv_block_4_bn1_gamma: Tensor[(128), float32], %separable_conv_block_4_bn1_beta: Tensor[(128), float32], %separable_conv_block_4_bn1_moving_mean: Tensor[(128), float32], %separable_conv_block_4_bn1_moving_var: Tensor[(128), float32], %separable_conv_block_4_conv2_weight: Tensor[(256, 128, 1, 1), float32], %separable_conv_block_4_bn2_gamma: Tensor[(256), float32], %separable_conv_block_4_bn2_beta: Tensor[(256), float32], %separable_conv_block_4_bn2_moving_mean: Tensor[(256), float32], %separable_conv_block_4_bn2_moving_var: Tensor[(256), float32], %separable_conv_block_5_weight: Tensor[(256, 1, 3, 3), float32], %separable_conv_block_5_bn1_gamma: Tensor[(256), float32], %separable_conv_block_5_bn1_beta: Tensor[(256), float32], %separable_conv_block_5_bn1_moving_mean: Tensor[(256), float32], %separable_conv_block_5_bn1_moving_var: Tensor[(256), float32], %separable_conv_block_5_conv2_weight: Tensor[(256, 256, 1, 1), float32], %separable_conv_block_5_bn2_gamma: Tensor[(256), float32], %separable_conv_block_5_bn2_beta: Tensor[(256), float32], %separable_conv_block_5_bn2_moving_mean: Tensor[(256), float32], %separable_conv_block_5_bn2_moving_var: Tensor[(256), float32], %separable_conv_block_6_weight: Tensor[(256, 1, 3, 3), float32], %separable_conv_block_6_bn1_gamma: Tensor[(256), float32], %separable_conv_block_6_bn1_beta: Tensor[(256), float32], %separable_conv_block_6_bn1_moving_mean: Tensor[(256), float32], %separable_conv_block_6_bn1_moving_var: Tensor[(256), float32], %separable_conv_block_6_conv2_weight: Tensor[(512, 256, 1, 1), float32], %separable_conv_block_6_bn2_gamma: Tensor[(512), float32], %separable_conv_block_6_bn2_beta: Tensor[(512), float32], %separable_conv_block_6_bn2_moving_mean: Tensor[(512), float32], %separable_conv_block_6_bn2_moving_var: Tensor[(512), float32], %separable_conv_block_7_weight: Tensor[(512, 1, 3, 3), float32], %separable_conv_block_7_bn1_gamma: Tensor[(512), float32], %separable_conv_block_7_bn1_beta: Tensor[(512), float32], %separable_conv_block_7_bn1_moving_mean: Tensor[(512), float32], %separable_conv_block_7_bn1_moving_var: Tensor[(512), float32], %separable_conv_block_7_conv2_weight: Tensor[(512, 512, 1, 1), float32], %separable_conv_block_7_bn2_gamma: Tensor[(512), float32], %separable_conv_block_7_bn2_beta: Tensor[(512), float32], %separable_conv_block_7_bn2_moving_mean: Tensor[(512), float32], %separable_conv_block_7_bn2_moving_var: Tensor[(512), float32], %separable_conv_block_8_weight: Tensor[(512, 1, 3, 3), float32], %separable_conv_block_8_bn1_gamma: Tensor[(512), float32], %separable_conv_block_8_bn1_beta: Tensor[(512), float32], %separable_conv_block_8_bn1_moving_mean: Tensor[(512), float32], %separable_conv_block_8_bn1_moving_var: Tensor[(512), float32], %separable_conv_block_8_conv2_weight: Tensor[(512, 512, 1, 1), float32], %separable_conv_block_8_bn2_gamma: Tensor[(512), float32], %separable_conv_block_8_bn2_beta: Tensor[(512), float32], %separable_conv_block_8_bn2_moving_mean: Tensor[(512), float32], %separable_conv_block_8_bn2_moving_var: Tensor[(512), float32], %separable_conv_block_9_weight: Tensor[(512, 1, 3, 3), float32], %separable_conv_block_9_bn1_gamma: Tensor[(512), float32], %separable_conv_block_9_bn1_beta: Tensor[(512), float32], %separable_conv_block_9_bn1_moving_mean: Tensor[(512), float32], %separable_conv_block_9_bn1_moving_var: Tensor[(512), float32], %separable_conv_block_9_conv2_weight: Tensor[(512, 512, 1, 1), float32], %separable_conv_block_9_bn2_gamma: Tensor[(512), float32], %separable_conv_block_9_bn2_beta: Tensor[(512), float32], %separable_conv_block_9_bn2_moving_mean: Tensor[(512), float32], %separable_conv_block_9_bn2_moving_var: Tensor[(512), float32], %separable_conv_block_10_weight: Tensor[(512, 1, 3, 3), float32], %separable_conv_block_10_bn1_gamma: Tensor[(512), float32], %separable_conv_block_10_bn1_beta: Tensor[(512), float32], %separable_conv_block_10_bn1_moving_mean: Tensor[(512), float32], %separable_conv_block_10_bn1_moving_var: Tensor[(512), float32], %separable_conv_block_10_conv2_weight: Tensor[(512, 512, 1, 1), float32], %separable_conv_block_10_bn2_gamma: Tensor[(512), float32], %separable_conv_block_10_bn2_beta: Tensor[(512), float32], %separable_conv_block_10_bn2_moving_mean: Tensor[(512), float32], %separable_conv_block_10_bn2_moving_var: Tensor[(512), float32], %separable_conv_block_11_weight: Tensor[(512, 1, 3, 3), float32], %separable_conv_block_11_bn1_gamma: Tensor[(512), float32], %separable_conv_block_11_bn1_beta: Tensor[(512), float32], %separable_conv_block_11_bn1_moving_mean: Tensor[(512), float32], %separable_conv_block_11_bn1_moving_var: Tensor[(512), float32], %separable_conv_block_11_conv2_weight: Tensor[(512, 512, 1, 1), float32], %separable_conv_block_11_bn2_gamma: Tensor[(512), float32], %separable_conv_block_11_bn2_beta: Tensor[(512), float32], %separable_conv_block_11_bn2_moving_mean: Tensor[(512), float32], %separable_conv_block_11_bn2_moving_var: Tensor[(512), float32], %separable_conv_block_12_weight: Tensor[(512, 1, 3, 3), float32], %separable_conv_block_12_bn1_gamma: Tensor[(512), float32], %separable_conv_block_12_bn1_beta: Tensor[(512), float32], %separable_conv_block_12_bn1_moving_mean: Tensor[(512), float32], %separable_conv_block_12_bn1_moving_var: Tensor[(512), float32], %separable_conv_block_12_conv2_weight: Tensor[(1024, 512, 1, 1), float32], %separable_conv_block_12_bn2_gamma: Tensor[(1024), float32], %separable_conv_block_12_bn2_beta: Tensor[(1024), float32], %separable_conv_block_12_bn2_moving_mean: Tensor[(1024), float32], %separable_conv_block_12_bn2_moving_var: Tensor[(1024), float32], %separable_conv_block_13_weight: Tensor[(1024, 1, 3, 3), float32], %separable_conv_block_13_bn1_gamma: Tensor[(1024), float32], %separable_conv_block_13_bn1_beta: Tensor[(1024), float32], %separable_conv_block_13_bn1_moving_mean: Tensor[(1024), float32], %separable_conv_block_13_bn1_moving_var: Tensor[(1024), float32], %separable_conv_block_13_conv2_weight: Tensor[(1024, 1024, 1, 1), float32], %separable_conv_block_13_bn2_gamma: Tensor[(1024), float32], %separable_conv_block_13_bn2_beta: Tensor[(1024), float32], %separable_conv_block_13_bn2_moving_mean: Tensor[(1024), float32], %separable_conv_block_13_bn2_moving_var: Tensor[(1024), float32], %fc_weight: Tensor[(1000, 1024), float32], %fc_bias: Tensor[(1000), float32]) -> Tensor[(1, 1000), float32] {
  %0 = nn.conv2d(%data, %conv_block_1_conv_weight, strides=[2, 2], padding=[1, 1, 1, 1], channels=32, kernel_size=[3, 3]) /* ty=Tensor[(1, 32, 112, 112), float32] */;
  %1 = nn.batch_norm(%0, %conv_block_1_bn_gamma, %conv_block_1_bn_beta, %conv_block_1_bn_moving_mean, %conv_block_1_bn_moving_var) /* ty=(Tensor[(1, 32, 112, 112), float32], Tensor[(32), float32], Tensor[(32), float32]) */;
  %2 = %1.0;
  %3 = nn.relu(%2) /* ty=Tensor[(1, 32, 112, 112), float32] */;
  %4 = nn.conv2d(%3, %separable_conv_block_1_weight, padding=[1, 1, 1, 1], groups=32, channels=32, kernel_size=[3, 3]) /* ty=Tensor[(1, 32, 112, 112), float32] */;
  %5 = nn.batch_norm(%4, %separable_conv_block_1_bn1_gamma, %separable_conv_block_1_bn1_beta, %separable_conv_block_1_bn1_moving_mean, %separable_conv_block_1_bn1_moving_var) /* ty=(Tensor[(1, 32, 112, 112), float32], Tensor[(32), float32], Tensor[(32), float32]) */;
  %6 = %5.0;
  %7 = nn.relu(%6) /* ty=Tensor[(1, 32, 112, 112), float32] */;
  %8 = nn.conv2d(%7, %separable_conv_block_1_conv2_weight, padding=[0, 0, 0, 0], channels=64, kernel_size=[1, 1]) /* ty=Tensor[(1, 64, 112, 112), float32] */;
  %9 = nn.batch_norm(%8, %separable_conv_block_1_bn2_gamma, %separable_conv_block_1_bn2_beta, %separable_conv_block_1_bn2_moving_mean, %separable_conv_block_1_bn2_moving_var) /* ty=(Tensor[(1, 64, 112, 112), float32], Tensor[(64), float32], Tensor[(64), float32]) */;
  %10 = %9.0;
  %11 = nn.relu(%10) /* ty=Tensor[(1, 64, 112, 112), float32] */;
  %12 = nn.conv2d(%11, %separable_conv_block_2_weight, strides=[2, 2], padding=[1, 1, 1, 1], groups=64, channels=64, kernel_size=[3, 3]) /* ty=Tensor[(1, 64, 56, 56), float32] */;
  %13 = nn.batch_norm(%12, %separable_conv_block_2_bn1_gamma, %separable_conv_block_2_bn1_beta, %separable_conv_block_2_bn1_moving_mean, %separable_conv_block_2_bn1_moving_var) /* ty=(Tensor[(1, 64, 56, 56), float32], Tensor[(64), float32], Tensor[(64), float32]) */;
  %14 = %13.0;
  %15 = nn.relu(%14) /* ty=Tensor[(1, 64, 56, 56), float32] */;
  %16 = nn.conv2d(%15, %separable_conv_block_2_conv2_weight, padding=[0, 0, 0, 0], channels=128, kernel_size=[1, 1]) /* ty=Tensor[(1, 128, 56, 56), float32] */;
  %17 = nn.batch_norm(%16, %separable_conv_block_2_bn2_gamma, %separable_conv_block_2_bn2_beta, %separable_conv_block_2_bn2_moving_mean, %separable_conv_block_2_bn2_moving_var) /* ty=(Tensor[(1, 128, 56, 56), float32], Tensor[(128), float32], Tensor[(128), float32]) */;
  %18 = %17.0;
  %19 = nn.relu(%18) /* ty=Tensor[(1, 128, 56, 56), float32] */;
  %20 = nn.conv2d(%19, %separable_conv_block_3_weight, padding=[1, 1, 1, 1], groups=128, channels=128, kernel_size=[3, 3]) /* ty=Tensor[(1, 128, 56, 56), float32] */;
  %21 = nn.batch_norm(%20, %separable_conv_block_3_bn1_gamma, %separable_conv_block_3_bn1_beta, %separable_conv_block_3_bn1_moving_mean, %separable_conv_block_3_bn1_moving_var) /* ty=(Tensor[(1, 128, 56, 56), float32], Tensor[(128), float32], Tensor[(128), float32]) */;
  %22 = %21.0;
  %23 = nn.relu(%22) /* ty=Tensor[(1, 128, 56, 56), float32] */;
  %24 = nn.conv2d(%23, %separable_conv_block_3_conv2_weight, padding=[0, 0, 0, 0], channels=128, kernel_size=[1, 1]) /* ty=Tensor[(1, 128, 56, 56), float32] */;
  %25 = nn.batch_norm(%24, %separable_conv_block_3_bn2_gamma, %separable_conv_block_3_bn2_beta, %separable_conv_block_3_bn2_moving_mean, %separable_conv_block_3_bn2_moving_var) /* ty=(Tensor[(1, 128, 56, 56), float32], Tensor[(128), float32], Tensor[(128), float32]) */;
  %26 = %25.0;
  %27 = nn.relu(%26) /* ty=Tensor[(1, 128, 56, 56), float32] */;
  %28 = nn.conv2d(%27, %separable_conv_block_4_weight, strides=[2, 2], padding=[1, 1, 1, 1], groups=128, channels=128, kernel_size=[3, 3]) /* ty=Tensor[(1, 128, 28, 28), float32] */;
  %29 = nn.batch_norm(%28, %separable_conv_block_4_bn1_gamma, %separable_conv_block_4_bn1_beta, %separable_conv_block_4_bn1_moving_mean, %separable_conv_block_4_bn1_moving_var) /* ty=(Tensor[(1, 128, 28, 28), float32], Tensor[(128), float32], Tensor[(128), float32]) */;
  %30 = %29.0;
  %31 = nn.relu(%30) /* ty=Tensor[(1, 128, 28, 28), float32] */;
  %32 = nn.conv2d(%31, %separable_conv_block_4_conv2_weight, padding=[0, 0, 0, 0], channels=256, kernel_size=[1, 1]) /* ty=Tensor[(1, 256, 28, 28), float32] */;
  %33 = nn.batch_norm(%32, %separable_conv_block_4_bn2_gamma, %separable_conv_block_4_bn2_beta, %separable_conv_block_4_bn2_moving_mean, %separable_conv_block_4_bn2_moving_var) /* ty=(Tensor[(1, 256, 28, 28), float32], Tensor[(256), float32], Tensor[(256), float32]) */;
  %34 = %33.0;
  %35 = nn.relu(%34) /* ty=Tensor[(1, 256, 28, 28), float32] */;
  %36 = nn.conv2d(%35, %separable_conv_block_5_weight, padding=[1, 1, 1, 1], groups=256, channels=256, kernel_size=[3, 3]) /* ty=Tensor[(1, 256, 28, 28), float32] */;
  %37 = nn.batch_norm(%36, %separable_conv_block_5_bn1_gamma, %separable_conv_block_5_bn1_beta, %separable_conv_block_5_bn1_moving_mean, %separable_conv_block_5_bn1_moving_var) /* ty=(Tensor[(1, 256, 28, 28), float32], Tensor[(256), float32], Tensor[(256), float32]) */;
  %38 = %37.0;
  %39 = nn.relu(%38) /* ty=Tensor[(1, 256, 28, 28), float32] */;
  %40 = nn.conv2d(%39, %separable_conv_block_5_conv2_weight, padding=[0, 0, 0, 0], channels=256, kernel_size=[1, 1]) /* ty=Tensor[(1, 256, 28, 28), float32] */;
  %41 = nn.batch_norm(%40, %separable_conv_block_5_bn2_gamma, %separable_conv_block_5_bn2_beta, %separable_conv_block_5_bn2_moving_mean, %separable_conv_block_5_bn2_moving_var) /* ty=(Tensor[(1, 256, 28, 28), float32], Tensor[(256), float32], Tensor[(256), float32]) */;
  %42 = %41.0;
  %43 = nn.relu(%42) /* ty=Tensor[(1, 256, 28, 28), float32] */;
  %44 = nn.conv2d(%43, %separable_conv_block_6_weight, strides=[2, 2], padding=[1, 1, 1, 1], groups=256, channels=256, kernel_size=[3, 3]) /* ty=Tensor[(1, 256, 14, 14), float32] */;
  %45 = nn.batch_norm(%44, %separable_conv_block_6_bn1_gamma, %separable_conv_block_6_bn1_beta, %separable_conv_block_6_bn1_moving_mean, %separable_conv_block_6_bn1_moving_var) /* ty=(Tensor[(1, 256, 14, 14), float32], Tensor[(256), float32], Tensor[(256), float32]) */;
  %46 = %45.0;
  %47 = nn.relu(%46) /* ty=Tensor[(1, 256, 14, 14), float32] */;
  %48 = nn.conv2d(%47, %separable_conv_block_6_conv2_weight, padding=[0, 0, 0, 0], channels=512, kernel_size=[1, 1]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %49 = nn.batch_norm(%48, %separable_conv_block_6_bn2_gamma, %separable_conv_block_6_bn2_beta, %separable_conv_block_6_bn2_moving_mean, %separable_conv_block_6_bn2_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %50 = %49.0;
  %51 = nn.relu(%50) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %52 = nn.conv2d(%51, %separable_conv_block_7_weight, padding=[1, 1, 1, 1], groups=512, channels=512, kernel_size=[3, 3]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %53 = nn.batch_norm(%52, %separable_conv_block_7_bn1_gamma, %separable_conv_block_7_bn1_beta, %separable_conv_block_7_bn1_moving_mean, %separable_conv_block_7_bn1_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %54 = %53.0;
  %55 = nn.relu(%54) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %56 = nn.conv2d(%55, %separable_conv_block_7_conv2_weight, padding=[0, 0, 0, 0], channels=512, kernel_size=[1, 1]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %57 = nn.batch_norm(%56, %separable_conv_block_7_bn2_gamma, %separable_conv_block_7_bn2_beta, %separable_conv_block_7_bn2_moving_mean, %separable_conv_block_7_bn2_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %58 = %57.0;
  %59 = nn.relu(%58) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %60 = nn.conv2d(%59, %separable_conv_block_8_weight, padding=[1, 1, 1, 1], groups=512, channels=512, kernel_size=[3, 3]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %61 = nn.batch_norm(%60, %separable_conv_block_8_bn1_gamma, %separable_conv_block_8_bn1_beta, %separable_conv_block_8_bn1_moving_mean, %separable_conv_block_8_bn1_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %62 = %61.0;
  %63 = nn.relu(%62) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %64 = nn.conv2d(%63, %separable_conv_block_8_conv2_weight, padding=[0, 0, 0, 0], channels=512, kernel_size=[1, 1]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %65 = nn.batch_norm(%64, %separable_conv_block_8_bn2_gamma, %separable_conv_block_8_bn2_beta, %separable_conv_block_8_bn2_moving_mean, %separable_conv_block_8_bn2_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %66 = %65.0;
  %67 = nn.relu(%66) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %68 = nn.conv2d(%67, %separable_conv_block_9_weight, padding=[1, 1, 1, 1], groups=512, channels=512, kernel_size=[3, 3]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %69 = nn.batch_norm(%68, %separable_conv_block_9_bn1_gamma, %separable_conv_block_9_bn1_beta, %separable_conv_block_9_bn1_moving_mean, %separable_conv_block_9_bn1_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %70 = %69.0;
  %71 = nn.relu(%70) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %72 = nn.conv2d(%71, %separable_conv_block_9_conv2_weight, padding=[0, 0, 0, 0], channels=512, kernel_size=[1, 1]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %73 = nn.batch_norm(%72, %separable_conv_block_9_bn2_gamma, %separable_conv_block_9_bn2_beta, %separable_conv_block_9_bn2_moving_mean, %separable_conv_block_9_bn2_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %74 = %73.0;
  %75 = nn.relu(%74) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %76 = nn.conv2d(%75, %separable_conv_block_10_weight, padding=[1, 1, 1, 1], groups=512, channels=512, kernel_size=[3, 3]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %77 = nn.batch_norm(%76, %separable_conv_block_10_bn1_gamma, %separable_conv_block_10_bn1_beta, %separable_conv_block_10_bn1_moving_mean, %separable_conv_block_10_bn1_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %78 = %77.0;
  %79 = nn.relu(%78) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %80 = nn.conv2d(%79, %separable_conv_block_10_conv2_weight, padding=[0, 0, 0, 0], channels=512, kernel_size=[1, 1]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %81 = nn.batch_norm(%80, %separable_conv_block_10_bn2_gamma, %separable_conv_block_10_bn2_beta, %separable_conv_block_10_bn2_moving_mean, %separable_conv_block_10_bn2_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %82 = %81.0;
  %83 = nn.relu(%82) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %84 = nn.conv2d(%83, %separable_conv_block_11_weight, padding=[1, 1, 1, 1], groups=512, channels=512, kernel_size=[3, 3]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %85 = nn.batch_norm(%84, %separable_conv_block_11_bn1_gamma, %separable_conv_block_11_bn1_beta, %separable_conv_block_11_bn1_moving_mean, %separable_conv_block_11_bn1_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %86 = %85.0;
  %87 = nn.relu(%86) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %88 = nn.conv2d(%87, %separable_conv_block_11_conv2_weight, padding=[0, 0, 0, 0], channels=512, kernel_size=[1, 1]) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %89 = nn.batch_norm(%88, %separable_conv_block_11_bn2_gamma, %separable_conv_block_11_bn2_beta, %separable_conv_block_11_bn2_moving_mean, %separable_conv_block_11_bn2_moving_var) /* ty=(Tensor[(1, 512, 14, 14), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %90 = %89.0;
  %91 = nn.relu(%90) /* ty=Tensor[(1, 512, 14, 14), float32] */;
  %92 = nn.conv2d(%91, %separable_conv_block_12_weight, strides=[2, 2], padding=[1, 1, 1, 1], groups=512, channels=512, kernel_size=[3, 3]) /* ty=Tensor[(1, 512, 7, 7), float32] */;
  %93 = nn.batch_norm(%92, %separable_conv_block_12_bn1_gamma, %separable_conv_block_12_bn1_beta, %separable_conv_block_12_bn1_moving_mean, %separable_conv_block_12_bn1_moving_var) /* ty=(Tensor[(1, 512, 7, 7), float32], Tensor[(512), float32], Tensor[(512), float32]) */;
  %94 = %93.0;
  %95 = nn.relu(%94) /* ty=Tensor[(1, 512, 7, 7), float32] */;
  %96 = nn.conv2d(%95, %separable_conv_block_12_conv2_weight, padding=[0, 0, 0, 0], channels=1024, kernel_size=[1, 1]) /* ty=Tensor[(1, 1024, 7, 7), float32] */;
  %97 = nn.batch_norm(%96, %separable_conv_block_12_bn2_gamma, %separable_conv_block_12_bn2_beta, %separable_conv_block_12_bn2_moving_mean, %separable_conv_block_12_bn2_moving_var) /* ty=(Tensor[(1, 1024, 7, 7), float32], Tensor[(1024), float32], Tensor[(1024), float32]) */;
  %98 = %97.0;
  %99 = nn.relu(%98) /* ty=Tensor[(1, 1024, 7, 7), float32] */;
  %100 = nn.conv2d(%99, %separable_conv_block_13_weight, padding=[1, 1, 1, 1], groups=1024, channels=1024, kernel_size=[3, 3]) /* ty=Tensor[(1, 1024, 7, 7), float32] */;
  %101 = nn.batch_norm(%100, %separable_conv_block_13_bn1_gamma, %separable_conv_block_13_bn1_beta, %separable_conv_block_13_bn1_moving_mean, %separable_conv_block_13_bn1_moving_var) /* ty=(Tensor[(1, 1024, 7, 7), float32], Tensor[(1024), float32], Tensor[(1024), float32]) */;
  %102 = %101.0;
  %103 = nn.relu(%102) /* ty=Tensor[(1, 1024, 7, 7), float32] */;
  %104 = nn.conv2d(%103, %separable_conv_block_13_conv2_weight, padding=[0, 0, 0, 0], channels=1024, kernel_size=[1, 1]) /* ty=Tensor[(1, 1024, 7, 7), float32] */;
  %105 = nn.batch_norm(%104, %separable_conv_block_13_bn2_gamma, %separable_conv_block_13_bn2_beta, %separable_conv_block_13_bn2_moving_mean, %separable_conv_block_13_bn2_moving_var) /* ty=(Tensor[(1, 1024, 7, 7), float32], Tensor[(1024), float32], Tensor[(1024), float32]) */;
  %106 = %105.0;
  %107 = nn.relu(%106) /* ty=Tensor[(1, 1024, 7, 7), float32] */;
  %108 = nn.global_avg_pool2d(%107) /* ty=Tensor[(1, 1024, 1, 1), float32] */;
  %109 = nn.batch_flatten(%108) /* ty=Tensor[(1, 1024), float32] */;
  %110 = nn.dense(%109, %fc_weight, units=1000) /* ty=Tensor[(1, 1000), float32] */;
  %111 = nn.bias_add(%110, %fc_bias) /* ty=Tensor[(1, 1000), float32] */;
  nn.softmax(%111) /* ty=Tensor[(1, 1000), float32] */
}
"#,
        r#"
(some-glenside-expr)
"#
    );
}