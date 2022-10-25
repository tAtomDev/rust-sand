pub fn iterate_grid_line<T>(x0: i32, y0: i32, x1: i32, y1: i32, mut callback: T) 
    where T: FnMut(i32, i32) -> ()
{
    if x0 == x1 && y0 == y1 {
        callback(x0, y0);
        return;
    }

    let x_diff: i32 = x0 - x1;
    let y_diff: i32 = y0 - y1;

    let x_diff_is_larger: bool = x_diff.abs() > y_diff.abs();

    let x_modifier: i32 = match x_diff.is_negative() {
        true => 1,
        _ => -1
    };

    let y_modifier: i32 = match y_diff.is_negative() {
        true => 1,
        _ => -1
    };

    let longer_side_length = y_diff.abs().max(x_diff.abs());
    let shorter_side_length = y_diff.abs().min(x_diff.abs());
    let slope: f32 = match shorter_side_length == 0 || longer_side_length == 0 {
        true => 0f32,
        _ => (longer_side_length / shorter_side_length) as f32
    };

    let mut shorter_side_increase: i32;
    for i in 1..=longer_side_length {
        shorter_side_increase = ((i as f32) * slope).floor() as i32;

        let (x_increase, y_increase) = match x_diff_is_larger {
            true => (i, shorter_side_increase),
            false => (shorter_side_increase, i)
        };

        let current_x = x0 + (x_increase * x_modifier);
        let current_y = y0 + (y_increase * y_modifier);
        callback(current_x, current_y);
    }
}