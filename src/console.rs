use colored::*;

pub fn print_tree(info: &crate::analyzer::MediaInfo) {
    println!("Format: {}", info.format.green());
    println!("\nStructure:");
    print_structure(&info.structure, "", 0);
}

fn print_structure(items: &[crate::analyzer::ElementInfo], prefix: &str, depth: usize) {
    const COLORS: &[fn(String) -> ColoredString] = &[
        |s| s.cyan(),          // 第一层
        |s| s.yellow(),        // 第二层
        |s| s.green(),         // 第三层
        |s| s.blue(),          // 第四层
        |s| s.magenta(),       // 第五层
        |s| s.red(),           // 第六层
        |s| s.bright_cyan(),   // 第七层
        |s| s.bright_yellow(), // 第八层
        |s| s.bright_green(),  // 第九层
        |s| s.bright_blue(),   // 第十层
    ];

    for (i, item) in items.iter().enumerate() {
        let is_last = i == items.len() - 1;
        let marker = if is_last { "`---" } else { "!---" };

        // 打印节点名称
        let color_fn = COLORS[depth % COLORS.len()];
        let colored_name = color_fn(item.name.clone());
        println!("{}{}{}", prefix, marker.bright_black(), colored_name);

        // 打印属性（使用缩进）
        let prop_prefix = format!("{}        ", prefix);

        // 先打印基本属性（offset/size）
        println!(
            "{}{}: {}",
            prop_prefix,
            "offset".bright_black(),
            item.offset.bright_black()
        );
        println!(
            "{}{}: {}",
            prop_prefix,
            "size".bright_black(),
            item.size.bright_black()
        );

        // 再打印其他属性，使用暗灰色
        const MAX_PROPERTIES: usize = 12;
        let other_props: Vec<_> = item.properties.iter().collect();

        for prop in other_props.iter().take(MAX_PROPERTIES) {
            println!(
                "{}{}:  {}",
                prop_prefix,
                prop.name,
                prop.readable_value.black()
            );
        }

        if other_props.len() > MAX_PROPERTIES {
            println!(
                "{}...:  {} more",
                prop_prefix,
                other_props.len() - MAX_PROPERTIES
            );
        }

        // 递归打印子节点
        let child_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}!   ", prefix)
        };

        print_structure(&item.children, &child_prefix, depth + 1);
    }
}
