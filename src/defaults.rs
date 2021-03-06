// Arbitrary list of public DNS servers
pub static DEFAULT_DNS_SERVERS: &'static [(&str, &str)] = &[
    // "Level3",
    ("209.244.0.3", "Level3"),
    ("209.244.0.4", "Level3"),
    // "Verisign",),
    ("64.6.64.6", "Verisign"),
    ("64.6.65.6", "Verisign"),
    // "Google",),
    ("8.8.8.8", "Google"),
    ("8.8.4.4", "Google"),
    // "DNS.WATCH",),
    ("84.200.69.80", "DNS.WATCH"),
    ("84.200.70.40", "DNS.WATCH"),
    // "OpenDNS Home",),
    ("208.67.222.222", "OpenDNS Home"),
    ("208.67.220.220", "OpenDNS Home"),
    // "SafeDNS",),
    ("195.46.39.39", "SafeDNS"),
    ("195.46.39.40", "SafeDNS"),
    // "Dyn",),
    ("216.146.35.35", "Dyn"),
    ("216.146.36.36", "Dyn"),
    // "FreeDNS",),
    ("37.235.1.174", "FreeDNS"),
    ("37.235.1.177", "FreeDNS"),
    // "Alternate DNS",),
    ("198.101.242.72", "Alternate DNS"),
    ("23.253.163.53", "Alternate DNS"),
];

pub static DEFAULT_RECORD_TYPES: &'static [&str] = &["a", "aaaa", "mx"];
