use super::color::Rgb;

impl Rgb {
    pub const NAMES: [&str; 141] = [
        "aliceblue",
        "antiquewhite",
        "aqua",
        "aquamarine",
        "azure",
        "beige",
        "bisque",
        "black",
        "blanchedalmond",
        "blue",
        "blueviolet",
        "brown",
        "burlywood",
        "cadetblue",
        "chartreuse",
        "chocolate",
        "coral",
        "cornflowerblue",
        "cornsilk",
        "crimson",
        "cyan",
        "darkblue",
        "darkcyan",
        "darkgoldenrod",
        "darkgray",
        "darkgreen",
        "darkkhaki",
        "darkmagenta",
        "darkolivegreen",
        "darkorange",
        "darkorchid",
        "darkred",
        "darksalmon",
        "darkseagreen",
        "darkslateblue",
        "darkslategray",
        "darkturquoise",
        "darkviolet",
        "deeppink",
        "deepskyblue",
        "dimgray",
        "dodgerblue",
        "firebrick",
        "floralwhite",
        "forestgreen",
        "fuchsia",
        "gainsboro",
        "ghostwhite",
        "gold",
        "goldenrod",
        "gray",
        "green",
        "greenyellow",
        "honeydew",
        "hotpink",
        "indianred",
        "indigo",
        "ivory",
        "khaki",
        "lavender",
        "lavenderblush",
        "lawngreen",
        "lemonchiffon",
        "lightblue",
        "lightcoral",
        "lightcyan",
        "lightgoldenrodyellow",
        "lightgray",
        "lightgreen",
        "lightpink",
        "lightsalmon",
        "lightseagreen",
        "lightskyblue",
        "lightslategray",
        "lightsteelblue",
        "lightyellow",
        "lime",
        "limegreen",
        "linen",
        "magenta",
        "maroon",
        "mediumaquamarine",
        "mediumblue",
        "mediumorchid",
        "mediumpurple",
        "mediumseagreen",
        "mediumslateblue",
        "mediumspringgreen",
        "mediumturquoise",
        "mediumvioletred",
        "midnightblue",
        "mintcream",
        "mistyrose",
        "moccasin",
        "navajowhite",
        "navy",
        "oldlace",
        "olive",
        "olivedrab",
        "orange",
        "orangered",
        "orchid",
        "palegoldenrod",
        "palegreen",
        "paleturquoise",
        "palevioletred",
        "papayawhip",
        "peachpuff",
        "peru",
        "pink",
        "plum",
        "powderblue",
        "purple",
        "rebeccapurple",
        "red",
        "rosybrown",
        "royalblue",
        "saddlebrown",
        "salmon",
        "sandybrown",
        "seagreen",
        "seashell",
        "sienna",
        "silver",
        "skyblue",
        "slateblue",
        "slategray",
        "snow",
        "springgreen",
        "steelblue",
        "tan",
        "teal",
        "thistle",
        "tomato",
        "turquoise",
        "violet",
        "wheat",
        "white",
        "whitesmoke",
        "yellow",
        "yellowgreen",
    ];

    const NAME_VALUES: [Rgb; 141] = [
        Rgb::new(240.0, 248.0, 255.0),
        Rgb::new(250.0, 235.0, 215.0),
        Rgb::new(0.0, 255.0, 255.0),
        Rgb::new(127.0, 255.0, 212.0),
        Rgb::new(240.0, 255.0, 255.0),
        Rgb::new(245.0, 245.0, 220.0),
        Rgb::new(255.0, 228.0, 196.0),
        Rgb::new(0.0, 0.0, 0.0),
        Rgb::new(255.0, 235.0, 205.0),
        Rgb::new(0.0, 0.0, 255.0),
        Rgb::new(138.0, 43.0, 226.0),
        Rgb::new(165.0, 42.0, 42.0),
        Rgb::new(222.0, 184.0, 135.0),
        Rgb::new(95.0, 158.0, 160.0),
        Rgb::new(127.0, 255.0, 0.0),
        Rgb::new(210.0, 105.0, 30.0),
        Rgb::new(255.0, 127.0, 80.0),
        Rgb::new(100.0, 149.0, 237.0),
        Rgb::new(255.0, 248.0, 220.0),
        Rgb::new(220.0, 20.0, 60.0),
        Rgb::new(0.0, 255.0, 255.0),
        Rgb::new(0.0, 0.0, 139.0),
        Rgb::new(0.0, 139.0, 139.0),
        Rgb::new(184.0, 134.0, 11.0),
        Rgb::new(169.0, 169.0, 169.0),
        Rgb::new(0.0, 100.0, 0.0),
        Rgb::new(189.0, 183.0, 107.0),
        Rgb::new(139.0, 0.0, 139.0),
        Rgb::new(85.0, 107.0, 47.0),
        Rgb::new(255.0, 140.0, 0.0),
        Rgb::new(153.0, 50.0, 204.0),
        Rgb::new(139.0, 0.0, 0.0),
        Rgb::new(233.0, 150.0, 122.0),
        Rgb::new(143.0, 188.0, 143.0),
        Rgb::new(72.0, 61.0, 139.0),
        Rgb::new(47.0, 79.0, 79.0),
        Rgb::new(0.0, 206.0, 209.0),
        Rgb::new(148.0, 0.0, 211.0),
        Rgb::new(255.0, 20.0, 147.0),
        Rgb::new(0.0, 191.0, 255.0),
        Rgb::new(105.0, 105.0, 105.0),
        Rgb::new(30.0, 144.0, 255.0),
        Rgb::new(178.0, 34.0, 34.0),
        Rgb::new(255.0, 250.0, 240.0),
        Rgb::new(34.0, 139.0, 34.0),
        Rgb::new(255.0, 0.0, 255.0),
        Rgb::new(220.0, 220.0, 220.0),
        Rgb::new(248.0, 248.0, 255.0),
        Rgb::new(255.0, 215.0, 0.0),
        Rgb::new(218.0, 165.0, 32.0),
        Rgb::new(128.0, 128.0, 128.0),
        Rgb::new(0.0, 128.0, 0.0),
        Rgb::new(173.0, 255.0, 47.0),
        Rgb::new(240.0, 255.0, 240.0),
        Rgb::new(255.0, 105.0, 180.0),
        Rgb::new(205.0, 92.0, 92.0),
        Rgb::new(75.0, 0.0, 130.0),
        Rgb::new(255.0, 255.0, 240.0),
        Rgb::new(240.0, 230.0, 140.0),
        Rgb::new(230.0, 230.0, 250.0),
        Rgb::new(255.0, 240.0, 245.0),
        Rgb::new(124.0, 252.0, 0.0),
        Rgb::new(255.0, 250.0, 205.0),
        Rgb::new(173.0, 216.0, 230.0),
        Rgb::new(240.0, 128.0, 128.0),
        Rgb::new(224.0, 255.0, 255.0),
        Rgb::new(250.0, 250.0, 210.0),
        Rgb::new(211.0, 211.0, 211.0),
        Rgb::new(144.0, 238.0, 144.0),
        Rgb::new(255.0, 182.0, 193.0),
        Rgb::new(255.0, 160.0, 122.0),
        Rgb::new(32.0, 178.0, 170.0),
        Rgb::new(135.0, 206.0, 250.0),
        Rgb::new(119.0, 136.0, 153.0),
        Rgb::new(176.0, 196.0, 222.0),
        Rgb::new(255.0, 255.0, 224.0),
        Rgb::new(0.0, 255.0, 0.0),
        Rgb::new(50.0, 205.0, 50.0),
        Rgb::new(250.0, 240.0, 230.0),
        Rgb::new(255.0, 0.0, 255.0),
        Rgb::new(128.0, 0.0, 0.0),
        Rgb::new(102.0, 205.0, 170.0),
        Rgb::new(0.0, 0.0, 205.0),
        Rgb::new(186.0, 85.0, 211.0),
        Rgb::new(147.0, 112.0, 219.0),
        Rgb::new(60.0, 179.0, 113.0),
        Rgb::new(123.0, 104.0, 238.0),
        Rgb::new(0.0, 250.0, 154.0),
        Rgb::new(72.0, 209.0, 204.0),
        Rgb::new(199.0, 21.0, 133.0),
        Rgb::new(25.0, 25.0, 112.0),
        Rgb::new(245.0, 255.0, 250.0),
        Rgb::new(255.0, 228.0, 225.0),
        Rgb::new(255.0, 228.0, 181.0),
        Rgb::new(255.0, 222.0, 173.0),
        Rgb::new(0.0, 0.0, 128.0),
        Rgb::new(253.0, 245.0, 230.0),
        Rgb::new(128.0, 128.0, 0.0),
        Rgb::new(107.0, 142.0, 35.0),
        Rgb::new(255.0, 165.0, 0.0),
        Rgb::new(255.0, 69.0, 0.0),
        Rgb::new(218.0, 112.0, 214.0),
        Rgb::new(238.0, 232.0, 170.0),
        Rgb::new(152.0, 251.0, 152.0),
        Rgb::new(175.0, 238.0, 238.0),
        Rgb::new(219.0, 112.0, 147.0),
        Rgb::new(255.0, 239.0, 213.0),
        Rgb::new(255.0, 218.0, 185.0),
        Rgb::new(205.0, 133.0, 63.0),
        Rgb::new(255.0, 192.0, 203.0),
        Rgb::new(221.0, 160.0, 221.0),
        Rgb::new(176.0, 224.0, 230.0),
        Rgb::new(128.0, 0.0, 128.0),
        Rgb::new(102.0, 51.0, 153.0),
        Rgb::new(255.0, 0.0, 0.0),
        Rgb::new(188.0, 143.0, 143.0),
        Rgb::new(65.0, 105.0, 225.0),
        Rgb::new(139.0, 69.0, 19.0),
        Rgb::new(250.0, 128.0, 114.0),
        Rgb::new(244.0, 164.0, 96.0),
        Rgb::new(46.0, 139.0, 87.0),
        Rgb::new(255.0, 245.0, 238.0),
        Rgb::new(160.0, 82.0, 45.0),
        Rgb::new(192.0, 192.0, 192.0),
        Rgb::new(135.0, 206.0, 235.0),
        Rgb::new(106.0, 90.0, 205.0),
        Rgb::new(112.0, 128.0, 144.0),
        Rgb::new(255.0, 250.0, 250.0),
        Rgb::new(0.0, 255.0, 127.0),
        Rgb::new(70.0, 130.0, 180.0),
        Rgb::new(210.0, 180.0, 140.0),
        Rgb::new(0.0, 128.0, 128.0),
        Rgb::new(216.0, 191.0, 216.0),
        Rgb::new(255.0, 99.0, 71.0),
        Rgb::new(64.0, 224.0, 208.0),
        Rgb::new(238.0, 130.0, 238.0),
        Rgb::new(245.0, 222.0, 179.0),
        Rgb::new(255.0, 255.0, 255.0),
        Rgb::new(245.0, 245.0, 245.0),
        Rgb::new(255.0, 255.0, 0.0),
        Rgb::new(154.0, 205.0, 50.0),
    ];

    pub fn from_name(name: impl AsRef<str>) -> Option<Rgb> {
        let name = name.as_ref();
        if let Some(idx) = Self::NAMES
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
        {
            Some(Self::NAME_VALUES[idx])
        } else {
            None
        }
    }
}
