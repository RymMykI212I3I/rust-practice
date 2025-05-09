use std::io::{self, Write};

fn draw_tree(num_triangles: usize) {
    if num_triangles == 0 {
        return;
    }

    let max_width = if num_triangles == 0 {
        3
    } else {
        2 * num_triangles + 3
    };

    let top_star_spaces = (max_width.saturating_sub(1)) / 2;
    println!("{}{}", " ".repeat(top_star_spaces), "*");

    let single_star_spaces = (max_width.saturating_sub(1)) / 2;//зірка
    println!("{}{}", " ".repeat(single_star_spaces), "*");

    let three_stars_spaces = (max_width.saturating_sub(3)) / 2;
    println!("{}{}", " ".repeat(three_stars_spaces), "***");

    if num_triangles > 0 {
        (0..num_triangles).for_each(|i| {
            let num_rows_in_segment = i + 3;

            (0..num_rows_in_segment).for_each(|j| {
                let current_width = 1 + 2 * j;
                let spaces = (max_width.saturating_sub(current_width)) / 2;
                println!("{}{}", " ".repeat(spaces), "*".repeat(current_width));
            });
        });
    }
}

fn main() {
    println!("Малюємо ялинку яка була данна з файла:");
    draw_tree(4);

    
}
