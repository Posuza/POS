use dioxus::prelude::*;

fn catmull_rom_to_bezier(points: &[(f32, f32)]) -> String {
    if points.len() < 2 {
        return String::new();
    }

    let mut d = format!("M{:.1},{:.1}", points[0].0, points[0].1);

    for i in 0..(points.len() - 1) {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < points.len() {
            points[i + 2]
        } else {
            points[points.len() - 1]
        };

        let c1x = p1.0 + (p2.0 - p0.0) / 6.0;
        let c1y = p1.1 + (p2.1 - p0.1) / 6.0;
        let c2x = p2.0 - (p3.0 - p1.0) / 6.0;
        let c2y = p2.1 - (p3.1 - p1.1) / 6.0;

        d.push_str(&format!(
            " C{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
            c1x, c1y, c2x, c2y, p2.0, p2.1
        ));
    }

    d
}

// Deterministic color generator: maps a series name to a stable color.
fn deterministic_color(name: &str) -> String {
    // FNV-1a 32-bit hash for simple deterministic hashing
    let mut hash: u32 = 2166136261u32;
    for b in name.as_bytes() {
        hash ^= *b as u32;
        hash = hash.wrapping_mul(16777619u32);
    }
    let hue = hash % 360;
    // return HSL color so we can produce many distinct colors reliably
    format!("hsl({hue},65%,45%)", hue = hue)
}

fn series_color(name: &str) -> String {
    let lname = name.to_lowercase();
    if lname.contains("sales") {
        "#2563eb".to_string()
    } else if lname.contains("items") || lname.contains("item") {
        "#7c3aed".to_string()
    } else if lname.contains("ticket") {
        "#0ea5a4".to_string()
    } else if lname.contains("stock") {
        "#10b981".to_string()
    } else if lname.contains("user") || lname.contains("users") {
        "#f59e0b".to_string()
    } else {
        deterministic_color(name)
    }
}

#[component]
pub fn SalesTrend(series: Vec<(String, Vec<f32>)>, x_labels: Option<Vec<String>>) -> Element {
    let width = 420.0_f32;
    let height = 160.0_f32;
    let pad = 36.0_f32; // room for y labels
                        // When multiple series are provided, compute global min/max across all series
    let mut global_max = f32::NEG_INFINITY;
    let mut global_min = f32::INFINITY;
    for (_name, s) in series.iter() {
        for v in s.iter() {
            if *v > global_max {
                global_max = *v;
            }
            if *v < global_min {
                global_min = *v;
            }
        }
    }
    if global_max == f32::NEG_INFINITY {
        global_max = 1.0;
    }
    if global_min == f32::INFINITY {
        global_min = 0.0;
    }
    let range = if (global_max - global_min).abs() < std::f32::EPSILON {
        1.0
    } else {
        global_max - global_min
    };

    // compute per-series coordinates and paths
    let mut all_coords: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut all_paths: Vec<String> = Vec::new();
    for (_name, s) in series.iter() {
        let coords: Vec<(f32, f32)> = s
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let x = pad
                    + i as f32
                        * ((width - pad * 2.0) / ((s.len().saturating_sub(1)) as f32).max(1.0));
                let y = pad + (1.0 - ((v - global_min) / range)) * (height - pad * 2.0);
                (x, y)
            })
            .collect();
        let d = catmull_rom_to_bezier(&coords);
        all_coords.push(coords);
        all_paths.push(d);
    }
    // pick a stable color per series name so colors remain consistent
    let mut stroke_colors: Vec<String> = Vec::new();
    for (name, _s) in series.iter() {
        stroke_colors.push(series_color(name));
    }
    let area_color = stroke_colors
        .get(0)
        .cloned()
        .unwrap_or_else(|| "#2563eb".to_string());
    let area_path = all_coords.get(0).and_then(|coords| {
        if coords.is_empty() {
            None
        } else {
            let mut d = format!("M{:.1},{:.1}", coords[0].0, coords[0].1);
            for (x, y) in coords.iter().skip(1) {
                d.push_str(&format!(" L{:.1},{:.1}", x, y));
            }
            let last_x = coords.last().unwrap().0;
            let first_x = coords[0].0;
            let bottom = height - pad;
            d.push_str(&format!(
                " L{:.1},{:.1} L{:.1},{:.1} Z",
                last_x, bottom, first_x, bottom
            ));
            Some(d)
        }
    });
    // latest value for first series shown on top-right
    let latest = series
        .get(0)
        .and_then(|(_name, s)| s.last())
        .cloned()
        .unwrap_or(0.0);
    let latest_s = format!("{:.2}", latest);
    // Precompute stringified positions and labels to avoid complex expressions inside RSX
    // Expand viewBox height so the legend (placed below the chart) is inside the SVG viewport.
    let view_box = format!("0 0 {:.1} {:.1}", width, height + 36.0);
    let pad_s = format!("{:.1}", pad);
    let width_minus_pad = width - pad;
    let width_minus_pad_s = format!("{:.1}", width_minus_pad);
    let pad_minus_8_s = format!("{:.1}", pad - 8.0);

    // y ticks (and gridlines) precomputed
    let ticks = 4usize;
    let mut y_ticks_s: Vec<(String, String)> = Vec::new(); // (label, ypos)
    for i in 0..=ticks {
        let t = i as f32 / ticks as f32;
        let val = global_min + (1.0 - t) * range;
        let ypos = pad + t * (height - pad * 2.0);
        y_ticks_s.push((format!("{:.0}", val), format!("{:.1}", ypos)));
    }

    // x labels positions precomputed (sample labels to avoid overlap)
    // store (short_label, x_pos_string, transform_string) so we can rotate labels
    let x_label_positions: Vec<(String, String, String)> = if let Some(labels) = &x_labels {
        let max_labels = 8usize;
        let len = labels.len();
        let step = ((len as f32) / (max_labels as f32)).ceil().max(1.0) as usize;
        let mut out: Vec<(String, String, String)> = Vec::new();
        for (i, l) in labels.iter().enumerate() {
            if i % step == 0 || i + 1 == len {
                let x = pad
                    + i as f32 * ((width - pad * 2.0) / (len.saturating_sub(1) as f32).max(1.0));
                // shorten label (date only or first token) to keep it compact
                let short = l.split_whitespace().next().unwrap_or(l).to_string();
                let x_s = format!("{:.1}", x);
                let y_label = height - pad + 18.0;
                let xform = format!("rotate(-30 {:.1} {:.1})", x, y_label);
                out.push((short, x_s, xform));
            }
        }
        out
    } else {
        vec![]
    };

    // precompute some layout strings used in the RSX
    let x_label_y_s = format!("{:.1}", height - pad + 18.0);
    let height_minus_pad_s = format!("{:.1}", height - pad);
    // sampled vertical grid x positions (stringified) for the sampled labels
    let vertical_xs: Vec<String> = x_label_positions
        .iter()
        .map(|(_, xpos, _)| xpos.clone())
        .collect();
    // place legend at the top inside the SVG (clamped to left padding)
    // tighten per-entry spacing so more legend items fit on one line
    let legend_entry_width = 72.0_f32;
    let mut legend_start_x = width - pad - (series.len() as f32 * legend_entry_width);
    if legend_start_x < pad {
        legend_start_x = pad;
    }
    let _legend_trans = format!("translate({:.1},{:.1})", legend_start_x, pad / 2.0);

    // adapt marker size for dense datasets (based on points in first series)
    let point_count = series.get(0).map(|(_name, s)| s.len()).unwrap_or(0);
    let marker_r = if point_count > 12 { 2.5 } else { 3.5 };
    // stringified marker radius not needed; use `marker_r` directly
    // tooltip / hover state (track series and index)
    let mut hover = use_signal(|| None as Option<(usize, usize)>);

    // per-series tooltip texts
    let mut tooltip_texts: Vec<Vec<String>> = Vec::new();
    for (_si, (_name, s)) in series.iter().enumerate() {
        let mut row: Vec<String> = Vec::new();
        for (i, v) in s.iter().enumerate() {
            let lbl = x_labels
                .as_ref()
                .and_then(|l| l.get(i))
                .cloned()
                .unwrap_or_else(|| format!("#{}", i + 1));
            row.push(format!("{} — {:.2}", lbl, v));
        }
        tooltip_texts.push(row);
    }

    rsx! {
        div { class: "sales-trend",
            div { class: "chart-row",
                /* Legend (wraps) */
                div { class: "chart-legend-wrapper",
                    { (0..series.len()).map(|i| {
                        let color = stroke_colors.get(i).cloned().unwrap_or_else(|| "var(--primary-color)".to_string());
                        let label = series.get(i).and_then(|(n,_)| Some(n.clone())).unwrap_or(format!("Series {}", i+1));
                        rsx!( div { class: "chart-legend-entry",
                            svg { width: "48", height: "16", view_box: "0 0 48 16",
                                line { x1: "2", y1: "8", x2: "34", y2: "8", stroke: "{color}", stroke_width: "2.5", stroke_linecap: "round" }
                                circle { cx: "18", cy: "8", r: "3.0", fill: "white", stroke: "{color}", stroke_width: "1.4" }
                            }
                            span { class: "chart-legend-label", "{label}" }
                        } )
                    }) }
                }

                /* Chart area */
                svg { class: "sales-chart", width: "100%", view_box: "{view_box}", preserve_aspect_ratio: "xMinYMid meet",
                    defs {
                        linearGradient { id: "sales-area-gradient", x1: "0", x2: "0", y1: "0", y2: "1",
                            stop { offset: "0%", stop_color: "{area_color}", stop_opacity: "0.28" }
                            stop { offset: "100%", stop_color: "{area_color}", stop_opacity: "0.02" }
                        }
                    }
                    { y_ticks_s.iter().map(|(lbl, ypos)| rsx!( g { line { x1: "{pad_s}", y1: "{ypos}", x2: "{width_minus_pad_s}", y2: "{ypos}", stroke: "#eef2f5", stroke_width: "1" } text { x: "{pad_minus_8_s}", y: "{ypos}", font_size: "9", text_anchor: "end", fill: "var(--muted-color)", "{lbl}" } } )) }

                    { vertical_xs.iter().map(|xpos| rsx!( line { x1: "{xpos}", x2: "{xpos}", y1: "{pad_s}", y2: "{height_minus_pad_s}", stroke: "#f3f5f7", stroke_width: "1" } )) }

                    { x_label_positions.iter().map(|(lbl, xpos, xform): &(String, String, String)| rsx!( text { x: "{xpos}", y: "{x_label_y_s}", transform: "{xform}", font_size: "8", text_anchor: "end", fill: "var(--muted-color)", "{lbl}" } )) }

                    line { x1: "{pad_s}", y1: "{pad}", x2: "{pad_s}", y2: "{height - pad}", stroke: "#e6e9ee", stroke_width: "1" }
                    line { x1: "{pad_s}", y1: "{height - pad}", x2: "{width_minus_pad_s}", y2: "{height - pad}", stroke: "#e6e9ee", stroke_width: "1" }

                    { area_path.as_ref().map(|d| rsx!( path { d: "{d}", fill: "url(#sales-area-gradient)", stroke: "none" } )) }

                    { all_paths.iter().enumerate().map(|(si,d)| {
                        let stroke_color = stroke_colors.get(si).cloned().unwrap_or_else(|| "var(--primary-color)".to_string());
                        let stroke_width = if si == 0 { "3.0" } else { "2.0" };
                        let dash = if si == 0 { "0" } else { "4 4" };
                        rsx!( path { d: "{d}", fill: "none", stroke: "{stroke_color}", stroke_width: "{stroke_width}", stroke_linecap: "round", stroke_linejoin: "round", stroke_dasharray: "{dash}" } )
                    }) }

                    { all_coords.iter().enumerate().flat_map(|(si,coords)| {
                        let color_for_series = stroke_colors.get(si).cloned().unwrap_or_else(|| "var(--primary-color)".to_string());
                        coords.iter().enumerate().map(move |(i,(xv,yv))| {
                            let x_s = format!("{:.1}", xv);
                            let y_s = format!("{:.1}", yv);
                            let color = color_for_series.as_str();
                            rsx!( circle { class: "chart-marker", cx: "{x_s}", cy: "{y_s}", r: "{marker_r}", fill: "white", stroke: "{color}", stroke_width: "1.5", onmouseenter: move |_| { hover.set(Some((si,i))); }, onmouseleave: move |_| { hover.set(None); } } )
                        })
                    }) }

                    {
                        let hover_val = *hover.read();
                        let tooltip_pre: Option<(String,String,String,String,String,String)> = if let Some((si, i)) = hover_val {
                            let tt = tooltip_texts.get(si).and_then(|r| r.get(i)).cloned().unwrap_or_default();
                            let est_w = (tt.len() as f32) * 7.2 + 12.0;
                            let est_h = 18.0_f32;
                            let mut tx = all_coords[si][i].0 + 6.0;
                            if tx + est_w > width - pad { tx = width - pad - est_w - 4.0; }
                            if tx < pad { tx = pad + 4.0; }
                            let ty = all_coords[si][i].1 - 12.0;
                            Some((format!("{:.1}", tx), format!("{:.1}", ty), format!("{:.1}", est_w), format!("{:.1}", est_h), tt, format!("{:.1}", ty + 12.0)))
                        } else { None };

                        tooltip_pre.as_ref().map(|(tx_s, ty_s, est_w_s, est_h_s, tt, text_y_s)| rsx!( g { class: "chart-tooltip", rect { x: "{tx_s}", y: "{ty_s}", width: "{est_w_s}", height: "{est_h_s}", rx: "6", fill: "var(--primary-color)" } text { x: "{tx_s}", y: "{text_y_s}", font_size: "11", fill: "white", "{tt}" } } ))
                    }

                    text { x: "{width_minus_pad_s}", y: "{pad_minus_8_s}", font_size: "11", text_anchor: "end", fill: "var(--muted-color)", "{latest_s}" }
                }
            }
        }
    }
}
