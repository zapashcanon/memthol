/*<LICENSE>
    This file is part of Memthol.

    Copyright (C) 2020 OCamlPro.

    Memthol is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Memthol is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Memthol.  If not, see <https://www.gnu.org/licenses/>.
*/

//! Header rendering stuff.

prelude! {}

/// Header of the UI.
///
/// Renders the info line and the settings.
pub struct Header {
    /// Link to the model.
    link: Link,
}

/// Height of the info of the header.
const HEADER_INFO_LINE_HEIGHT_PX: usize = 60;
/// Vertical button padding.
const HEADER_VERTICAL_LEFT_BUTTON_PADDING: usize = HEADER_INFO_LINE_HEIGHT_PX / 10;
/// Height of the info of the header.
const HEADER_INFO_LINE_LEFT_BUTTON_HEIGHT_PX: usize =
    HEADER_INFO_LINE_HEIGHT_PX - HEADER_VERTICAL_LEFT_BUTTON_PADDING;
/// Vertical button padding.
const HEADER_VERTICAL_BUTTON_PADDING: usize = HEADER_INFO_LINE_HEIGHT_PX / 4;
/// Height of the info of the header.
pub const HEADER_INFO_LINE_BUTTON_HEIGHT_PX: usize =
    HEADER_INFO_LINE_HEIGHT_PX - HEADER_VERTICAL_BUTTON_PADDING;
/// Height of a line of the header.
pub const HEADER_LINE_HEIGHT_PX: usize = 40;
/// Font size in the header.
const FONT_SIZE: usize = 130;

impl Header {
    /// Constructor.
    pub fn new(link: Link) -> Self {
        Self { link }
    }

    /// Height of the header in pixels.
    pub fn height_px(&self, model: &Model) -> usize {
        HEADER_INFO_LINE_HEIGHT_PX + model.settings.line_count() * HEADER_LINE_HEIGHT_PX
    }

    /// Renders the top header.
    pub fn render(&self, model: &Model) -> Html {
        let header_style = inline_css! {
            font({FONT_SIZE}%),

            width(100%),
            height({self.height_px(model)}px),

            fg({layout::LIGHT_BLUE_FG}),
            bg({layout::DARK_GREY_BG}),

            border_radius(0 px, 0 px, 20 px, 20 px),

            fixed(top),

            z_index(600),
        };

        html! {
            <header
                style = header_style
            >
                {model.settings.render(model)}
                {self.info_line(model)}
            </header>
        }
    }

    /// Generates the line containing generic info about the dump(s) and the OCP logo.
    pub fn info_line(&self, model: &Model) -> Html {
        define_style! {
            INFO_LINE = {
                height({HEADER_INFO_LINE_HEIGHT_PX}px)
            };
            LOGO = {
                height({HEADER_INFO_LINE_HEIGHT_PX - HEADER_INFO_LINE_HEIGHT_PX / 20}px)
            };
        }
        Self::three_part_line_with(
            &*INFO_LINE,
            // Left column.
            Self::center(self.info_line_buttons(model)),
            // Center column.
            Self::center(self.format_stats(model)),
            // Right column.
            Self::center(html! {
                <a
                    href = "https://www.ocamlpro.com"
                    target = "_blank"
                    style = LOGO
                >
                    { ocp_pic() }
                </a>
            }),
        )
    }

    /// Generates the buttons for the info line.
    pub fn info_line_buttons(&self, model: &Model) -> Html {
        define_style! {
            LEFT = {
                float(left),
                padding({HEADER_VERTICAL_LEFT_BUTTON_PADDING / 2}px, 0%),
            };
            RIGHT = {
                float(right),
                padding({HEADER_VERTICAL_BUTTON_PADDING / 2}px, 0%),
            };
        }

        let collapse = layout::button::img::collapse(
            Some(HEADER_INFO_LINE_LEFT_BUTTON_HEIGHT_PX),
            "collapsed_settings",
            if model.settings.can_collapse() {
                Some(
                    self.link
                        .callback(move |_| msg::Msg::from(msg::settings::Msg::Collapse)),
                )
            } else {
                None
            },
            "collapses the settings menu",
        );
        let expand = layout::button::img::expand(
            Some(HEADER_INFO_LINE_LEFT_BUTTON_HEIGHT_PX),
            "expand settings",
            if model.settings.can_expand() {
                Some(
                    self.link
                        .callback(move |_| msg::Msg::from(msg::settings::Msg::Expand)),
                )
            } else {
                None
            },
            "expands the settings menu",
        );

        html! {
            <>
                <div
                    style = LEFT
                >
                    {expand}
                </div>
                <div
                    style = LEFT
                >
                    {collapse}
                </div>
                <div
                    style = RIGHT
                >
                    {model.settings.buttons()}
                </div>
            </>
        }
    }

    /// Centers its content using the `table`/`table cell` trick.
    pub fn center(inner: Html) -> Html {
        define_style! {
            TABLE_STYLE = {
                table,
                width(100%),
                height(100%),
            };
            CENTER_CONTENT_STYLE = {
                table cell,
                padding(0%, 10 px),
                white_space(nowrap),
                text_align(center),
                vertical_align(middle),
            };
        }
        html! {
            <div
                style = TABLE_STYLE
            >
                <div
                    style = CENTER_CONTENT_STYLE
                >
                    {inner}
                </div>
            </div>
        }
    }

    /// Creates a header line with three columns (15%-70%-15%, float left).
    pub fn three_part_line_with(style: &'static str, lft: Html, center: Html, rgt: Html) -> Html {
        const center_width_pc: usize = 70;
        const lft_width_pc: usize = (100 - center_width_pc) / 2;
        const rgt_width_pc: usize = lft_width_pc;

        define_style! {
            tile_style! = {
                block,
                float(left),
                height(100%),
                overflow(scroll),
            };
            LFT_STYLE = {
                extends(tile_style),
                width({lft_width_pc}%),
            };
            CENTER_STYLE = {
                extends(tile_style),
                block,
                width({center_width_pc}%),
            };
            RGT_STYLE = {
                extends(tile_style),
                width({rgt_width_pc}%),
            };
        }

        html! {
            <div
                style = style
            >
                <div
                    style = LFT_STYLE
                >
                    {lft}
                </div>
                <div
                    style = CENTER_STYLE
                >
                    {center}
                </div>
                <div
                    style = RGT_STYLE
                >
                    {rgt}
                </div>
            </div>
        }
    }

    /// Formats the statistics, if any.
    fn format_stats(&self, model: &Model) -> Html {
        static LOCAL: time::chrono::Local = time::chrono::Local;

        define_style! {
            TXT_STYLE = {
                margin(0 px),
            };
            EMPH_STYLE = {
                fg({"#c8f5fd"}),
            };
        }

        if let Some(stats) = model.alloc_stats.as_ref() {
            let start = stats.start_date.date().with_timezone(&LOCAL);
            html! {
                <p
                    style = TXT_STYLE
                >
                    {"run started at "}
                    {emph(format!("{} (LT)", start.time().format("%H:%M:%S")))}
                    {", on "}
                    {emph(start.date().naive_local())}
                    {", ran for "}
                    {emph(stats.duration)}
                    {" with "}
                    {emph(num_fmt::str_do(stats.alloc_count as f64, identity))}
                    {" allocations, "}
                    {emph(num_fmt::bin_str_do(stats.total_size as f64, |mut s| {s.push('B') ; s}))}
                    {" | "}
                    {code(stats.dump_dir.display())}
                </p>
            }
        } else {
            html! {
                "No allocation statistics available at this moment..."
            }
        }
    }
}

/// Produces emphasized text.
pub fn emph(s: impl std::fmt::Display) -> Html {
    define_style! {
        EMPH_STYLE = {
            fg({"#c8f5fd"}),
        };
    }
    html! {
        <b
            style = EMPH_STYLE
        >
            {s}
        </b>
    }
}

/// Formats some text in code style.
pub fn code(s: impl std::fmt::Display) -> Html {
    define_style! {
        CODE_STYLE = {
            font(code, 100%),
            fg({"#ffb961"}),
        };
    }
    html! {
        <b
            style = CODE_STYLE
        >
            {s}
        </b>
    }
}

fn ocp_pic() -> Html {
    define_style! {
        STYLE = {
            width(auto),
            height({HEADER_INFO_LINE_HEIGHT_PX - HEADER_INFO_LINE_HEIGHT_PX / 20}px)
        };
    }
    html! {
        <svg
            style = STYLE
            viewBox = "0 0 24205 6969"
            fill = "none"
            xmlns = "http://www.w3.org/2000/svg"
        >
            <path d="M16889.9 1909.06C16614.8 1909.06 16391.1 2132.57 16391.1 2407.51V4651.96C16391.1 4926.9 16614.8 5148.95 16889.9 5148.95H22899.3C23174.4 5148.95 23398 4926.9 23398 4651.96V2407.51C23398 2132.57 23174.4 1909.06 22899.3 1909.06H16889.9ZM21854 2407.51C22330.7 2407.51 22728.2 2778.69 22728.2 3488.44C22715 4142.32 22310.7 4651.96 21659.7 4651.96C21248.7 4651.96 20824.6 4276.81 20824.6 3563.78C20824.6 2883.61 21262.2 2407.51 21854 2407.51ZM17780 2426.34H17816.3C18145.1 2426.34 18542.6 2561.62 18542.6 3097.22C18542.6 3501.37 18187.9 3786.92 17753.9 3786.92C17694.8 3786.92 17639.1 3781.23 17580 3768.09C17586.5 4099.96 17595.8 4506.63 17609 4608.49H17250.9V4556.32C17250.9 4270.46 17258.5 3987.77 17255.2 3734.76C17255.2 3179.45 17283.7 2651 17165.3 2503.14L17188.5 2446.63L17780 2426.34H17780ZM19634.3 2430.69C19963.1 2430.69 20362.1 2561.62 20362.1 3097.22C20362.1 3389.66 20177.9 3622.76 19908.3 3724.62C19987.2 3764.05 20571.6 4539.48 20663.7 4608.49H20220C20101.7 4368.62 19782.4 4023.5 19572 3786.92C19512.8 3786.92 19457.2 3781.23 19398 3768.09C19404.6 4099.96 19413.9 4506.63 19427 4608.49H19068.9V4556.32C19068.9 4270.46 19076.5 3987.77 19073.3 3734.76C19073.3 3179.45 19101.7 2651 18983.4 2503.14L19006.6 2446.63C19611.5 2430.2 19621.2 2430.69 19634.3 2430.69ZM17767 2610.36L17590.1 2617.61C17580.2 2735.9 17580 3198.51 17580 3494.23V3592.76C17626 3615.76 17682.1 3623.19 17787.3 3623.19C18040.4 3623.19 18251.2 3448.96 18251.2 3169.66C18251.2 2752.36 17974.1 2603.79 17767 2610.36ZM19585 2610.36L19408.2 2617.61C19398.3 2735.9 19398 3198.51 19398 3494.23V3592.76C19444 3615.76 19500.1 3623.19 19605.3 3623.19C19858.5 3623.19 20069.3 3448.96 20069.3 3169.66C20069.3 2745.79 19792.2 2603.79 19585 2610.36ZM21739.4 2610.36C21420.5 2610.36 21179.8 2991.42 21179.8 3458.01C21179.8 3967.31 21459.4 4454.9 21817.7 4454.9C22133.3 4454.9 22370.1 4069.88 22370.1 3600.01C22370.1 3057.84 22078.1 2610.36 21739.4 2610.36Z" fill="white"/>
            <path d="M7361.69 2407.05C7838.42 2407.05 8236.25 2778.42 8236.25 3488.3C8223.09 4142.3 7818.7 4651.7 7167.72 4651.7C6756.74 4651.7 6332.62 4277.05 6332.62 3563.89C6332.62 2883.59 6769.89 2407.05 7361.69 2407.05V2407.05ZM7325.53 4454.52C7641.16 4454.52 7877.88 4070 7877.88 3600.04C7877.88 3057.77 7585.26 2610.82 7246.62 2610.82C6927.71 2610.82 6687.7 2992.04 6687.7 3458.72C6687.7 3968.12 6967.16 4454.52 7325.53 4454.52" fill="white"/>
            <path d="M10264.8 4109.44C10146.4 4326.34 9866.98 4428.22 9676.28 4428.22C9179.83 4428.22 8939.82 3945.12 8939.82 3432.43C8939.82 3005.19 9137.09 2604.24 9590.8 2604.24C9889.99 2604.24 10011.6 2850.73 10011.6 2936.17L10064.2 2978.9C10179.3 2929.6 10271.4 2745.56 10271.4 2679.83C10199 2531.94 9913 2407.05 9630.26 2407.05C8999 2407.05 8578.16 2890.16 8578.16 3573.75C8578.16 4185.03 8913.52 4651.7 9531.62 4651.7C9942.59 4651.7 10277.9 4464.38 10277.9 4178.45C10277.9 4158.74 10274.7 4129.16 10264.8 4109.44" fill="white"/>
            <path d="M11432 4257.33C11399.1 4326.34 11247.9 4415.08 11083.5 4415.08C10902.6 4415.08 10827 4283.62 10827 4171.88C10827 3853.09 11438.6 3846.52 11441.8 3846.52C11441.8 3846.52 11432 4188.31 11432 4257.33V4257.33ZM11780.5 4392.07C11750.9 4329.63 11734.5 4168.59 11734.5 4050.28C11734.5 3550.74 11741 3481.72 11741 3412.71C11741 3077.49 11507.6 2942.75 11145.9 2942.75C10892.8 2942.75 10619.9 3047.91 10619.9 3245.1C10619.9 3340.41 10695.5 3392.99 10777.7 3392.99C10817.2 3392.99 10856.6 3383.13 10889.5 3360.13C10889.5 3350.27 10886.2 3337.12 10886.2 3327.26C10886.2 3208.95 10984.8 3093.92 11139.4 3093.92C11405.7 3093.92 11448.4 3310.83 11448.4 3570.46C11448.4 3632.9 11445.1 3701.92 11441.8 3767.65C11359.6 3747.93 11277.5 3734.78 11195.3 3734.78C10840.2 3734.78 10531.1 3908.96 10531.1 4227.75C10531.1 4438.08 10672.5 4651.7 10981.6 4651.7C11155.8 4651.7 11389.2 4530.1 11451.7 4329.63C11451.7 4378.93 11481.3 4651.7 11701.6 4651.7C11796.9 4651.7 11944.9 4608.98 12007.3 4569.54V4510.38C11997.5 4510.38 11843 4516.96 11780.5 4392.07" fill="white"/>
            <path d="M14709.9 4608.98C14650.7 4329.63 14683.6 3471.87 14677 3333.83C14670.5 3133.36 14512.6 2942.75 14282.5 2942.75C13973.5 2942.75 13779.5 3133.36 13677.6 3287.82C13648 3103.78 13496.7 2942.75 13286.3 2942.75C12964.1 2942.75 12753.7 3153.08 12655.1 3317.4C12664.9 3254.96 12671.5 3225.38 12671.5 3225.38C12681.4 3074.2 12661.6 2949.32 12503.8 2949.32C12369 2949.32 12270.4 2995.33 12211.2 3031.48V3090.64C12211.2 3090.64 12385.5 3103.78 12378.9 3488.3L12365.7 4595.83V4608.98H12694.5C12681.4 4546.54 12668.2 4461.09 12668.2 4362.49C12661.6 4014.13 12661.6 3560.6 12661.6 3425.86C12734 3340.41 12885.2 3202.38 13125.2 3202.38C13335.6 3205.66 13394.8 3346.98 13391.5 3662.48L13384.9 4451.23C13381.7 4510.38 13384.9 4562.97 13391.5 4608.98H13713.7C13657.8 4359.21 13677.6 3590.18 13680.8 3389.7C13756.5 3310.83 13901.1 3202.38 14121.4 3202.38C14331.8 3205.66 14391 3346.98 14387.7 3662.48L14381.1 4451.23C14377.9 4510.38 14381.1 4562.97 14387.7 4608.98H14709.9" fill="white"/>
            <path d="M15630.5 4608.97C15623.9 4572.82 15614 4526.81 15610.7 4470.94C15594.3 4106.14 15584.4 3136.64 15614 2660.1C15637 2305.16 15640.3 2127.7 15416.8 2127.7C15258.9 2127.7 15147.2 2209.86 15147.2 2209.86V2269.01C15242.5 2282.16 15301.7 2380.75 15301.7 2564.79C15301.7 2564.79 15308.3 3744.63 15301.7 4559.67V4608.97H15630.5" fill="white"/>
            <path d="M3820.31 5786.03C3822.69 5784.66 3825.01 5783.22 3827.27 5781.71L4769.86 5237.77C4809.63 5214.82 4834.13 5172.39 4834.13 5126.47V1840.59C4834.13 1794.68 4809.64 1752.26 4769.88 1729.31L3123.7 778.842C2929.5 666.714 2690.27 666.554 2495.91 778.421L1800.99 1178.41C1798.89 1179.48 1796.81 1180.6 1794.75 1181.79L845.52 1730.14C805.777 1753.1 781.297 1795.51 781.297 1841.41V5126.47C781.297 5172.39 805.799 5214.82 845.57 5237.77L1788.16 5781.71C1790.42 5783.22 1792.74 5784.66 1795.11 5786.03L2493.57 6189.1C2687.98 6301.28 2927.45 6301.28 3121.85 6189.1L3820.31 5786.03Z" fill="#CD5119" stroke="#832B03" stroke-width="257" stroke-linejoin="round"/>
            <path d="M3757.05 3479.94L2808.56 4027.55L2808.38 3194.33C2808.34 3049.63 2909.9 2873.68 3035.22 2801.33L3719.57 2406.22C3740.14 2394.35 3756.81 2403.97 3756.82 2427.72L3757.05 3479.94Z" fill="#F88834" stroke="#832B03" stroke-width="60" stroke-linejoin="round"/>
            <path d="M3757.09 4575.17L2822.45 5114.78C2814.8 5119.2 2808.59 5115.62 2808.59 5106.78L2808.35 4027.56L3756.84 3479.95L3757.09 4575.17Z" fill="#832B03" stroke="#832B03" stroke-width="60" stroke-linejoin="round"/>
            <path d="M3756.38 2388.67L3058.22 1985.39C2903.47 1895.99 2712.76 1896 2558.01 1985.4L2109.85 2244.31C1955.22 2333.65 1859.97 2498.67 1859.97 2677.26L1859.97 4575.33L2802.36 5119.41C2805.02 5120.95 2808.36 5119.03 2808.36 5115.95L2808.36 3225.1C2808.36 3046.55 2903.57 2881.55 3058.16 2792.2L3756.38 2388.67Z" fill="#F88834" stroke="#832B03" stroke-width="60" stroke-linejoin="round"/>
            <path d="M4755.47 2964.39L3757.67 2388.32L3757.19 4564.48L4754.98 5140.56L4755.47 2964.39Z" fill="#832B03" stroke="#832B03" stroke-width="60" stroke-linejoin="round"/>
            <path d="M3834.76 5826.31C3837.53 5824.71 3840.24 5823.04 3842.9 5821.3L4784.3 5278.05C4838.46 5246.79 4871.83 5189.01 4871.83 5126.47V1840.59C4871.83 1778.07 4838.48 1720.3 4784.34 1689.04L3138.16 738.573C2929.59 618.148 2672.65 617.976 2463.92 738.12L1769.99 1137.54C1767.54 1138.8 1765.11 1140.13 1762.7 1141.52L813.463 1689.87C759.339 1721.14 726 1778.9 726 1841.41V5126.47C726 5189.01 759.368 5246.79 813.531 5278.05L1754.93 5821.3C1757.59 5823.04 1760.31 5824.71 1763.08 5826.31L2461.54 6229.37C2670.32 6349.85 2927.51 6349.85 3136.3 6229.37L3834.76 5826.31Z" stroke="#832B03" stroke-width="350" stroke-linejoin="round"/>
        </svg>
    }
}
