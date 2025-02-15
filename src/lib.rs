//! # IADB API Wrapper
//! 
//! `iadb-api` is a wrapper for the Band of England's Statistical Interactive Database (IADB). The data can be accessed using specific series codes,
//! most of which are defined in the variants of the `SeriesCode` enum.
//! 
//! **Disclaimer:** This crate is an unofficial IADB wrapper, the maintainers of the crate are independent developers.
//! The developers of the crate do not accept any responsibility or liability for the accuracy, security, or completeness of the code,
//! or the information provided within the crate.
//! 
//! # Example
//! 
//! ```rust
//! use iadb_api::{schemas::IADBSeries, backend::IADB};
//! 
//! #[tokio::main]
//! async fn main() -> () {
//! 
//!     // Parameters
//!     let series_code: String = SeriesCode::IUDSOIA.to_string();
//!     let date_from: String = String::from("01/Jan/2000");
//!     let date_to: String = String::from("01/Oct/2018");
//! 
//!     // Data collection
//!     let data: IADBSeries = IADB::get_data(&series_code, &date_from, &date_to).await.unwrap();
//! 
//!     println!("{}", data);
//! 
//! }
//! ```
//! 
//! # General information
//! If you would like to add a commit or an issue, please do so using the GitHub link to the project:
//! - <https://github.com/rsadykhov/iadb-api>


//Re-Exports
pub use self::backend::IADB;
pub use self::schemas::{IADBSeries, IADBDataPoint};


pub mod schemas;
pub mod utils;
pub mod backend;



use std::fmt;


pub const BASE_URL: &str = "http://www.bankofengland.co.uk/boeapps/iadb/fromshowcolumns.asp";


/// Series code that identifies the data series in the Bank of England Database (IADB).
/// 
/// Series Codes: <https://www.bankofengland.co.uk/boeapps/database/index.asp?SectionRequired=I&first=yes&HideNums=-1&ExtraInfo=true&Travel=NIxIRx&levels=2>
pub enum SeriesCode {
    /// Daily Sterling overnight index average (SONIA) rate
    IUDSOIA,
    /// Spot exchange rate, Canadian Dollar into Sterling
    XUDLCDS,
    /// Spot exchange rate, Danish Krone into Sterling
    XUDLDKS,
    /// Spot exchange rate, Euro into Sterling
    XUDLERS,
    /// Spot exchange rate, Japanese Yen into Sterling
    XUDLJYS,
    /// Spot exchange rate, Norwegian Krone into Sterling
    XUDLNKS,
    /// Spot exchange rate, Swiss Franc into Sterling
    XUDLSFS,
    /// Spot exchange rate, Singapore Dollar into Sterling
    XUDLSGS,
    /// Spot exchange rate, Swedish Krona into Sterling
    XUDLSKS,
    /// Spot exchange rate, US $ into Sterling
    XUDLUSS,
    /// Wholesale interest and discount rates, Official Bank Rate, Daily
    IUDBEDR,
    /// Wholesale interest and discount rates, Average of UK banks' base rates, Daily
    IUDAMIH,
    /// US dollar forward premium/discount rates, 1 month, Daily
    XUDLDF1,
    /// US dollar forward premium/discount rates, 3 month, Daily
    XUDLDF3,
    /// US dollar forward premium/discount rates, 6 month, Daily
    XUDLDF6,
    /// US dollar forward premium/discount rates, 1 year, Daily
    XUDLDFY,
    /// £ sterling against US dollar forward rates, 1 month, Daily
    XUDLDS1,
    /// £ sterling against US dollar forward rates, 3 month, Daily
    XUDLDS3,
    /// £ sterling against US dollar forward rates, 6 month, Daily
    XUDLDS6,
    /// £ sterling against US dollar forward rates, 1 year, Daily
    XUDLDSY,
    /// Gold price, against £ sterling, Daily
    XUDLGPS,
    /// Gold price, against US dollar, Daily
    XUDLGPD,
    /// Yields, British Government Securities (calculated using VRP model), Nominal par yields, 5 year, Daily
    IUDSNPY,
    /// Yields, British Government Securities (calculated using VRP model), Nominal par yields, 10 year, Daily
    IUDMNPY,
    /// Yields, British Government Securities (calculated using VRP model), Nominal par yields, 20 year, Daily
    IUDLNPY,
    /// Yields, British Government Securities (calculated using VRP model), Zero coupon yields, Nominal, 5 year, Daily
    IUDSIZC,
    /// Yields, British Government Securities (calculated using VRP model), Zero coupon yields, Nominal, 10 year, Daily
    IUDMIZC,
    /// Yields, British Government Securities (calculated using VRP model), Zero coupon yields, Nominal, 20 year, Daily
    IUDLIZC,
    /// Yields, British Government Securities (calculated using VRP model), Implied forward yields, Nominal, 5 year, Daily
    IUDSIIF,
    /// Yields, British Government Securities (calculated using VRP model), Implied forward yields, Nominal, 10 year, Daily
    IUDMIIF,
    /// Yields, British Government Securities (calculated using VRP model), Implied forward yields, Nominal, 20 year, Daily
    IUDLIIF,
    /// Yields, British Government Securities (calculated using VRP model), Real gross redemption yields, 3.5% War Loan, Daily
    IUDWRLN,
    /// Yields, British Government Securities (calculated using VRP model), Real gross redemption yields, 2% Index Linked Treasury Stock 2006, Daily
    IUDAJUR,
    /// Yields, British Government Securities (calculated using VRP model), Real gross redemption yields, BoE Treasury Note 4.5% to 2004, Daily
    IUDEBEN,
    /// Yields, British Government Securities (calculated using VRP model), Real gross redemption yields, 2.5% Index Linked Treasury Stock 2016, Daily
    IUDAJLT,
    /// Yields, British Government Securities (calculated using VRP model), Real gross redemption yields, 2.75% BoE Euro Note 2006, Daily
    IUDBK58,
    /// Yields, British Government Securities (calculated using VRP model), Real gross redemption yields, 10 year par gross redemption yield on British Government Securities, Daily
    IUDAJLW,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 2 year, 60% LTV, Monthly
    IUMZICQ,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 2 year, 75% LTV, Combined bank and building society, Monthly
    IUMBV34,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 2 year, 85% LTV, Monthly
    IUMZICR,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 	2 year, 90% LTV, Combined bank and building society, Monthly
    IUMB482,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 2 year, 95% LTV, Combined bank and building society, Monthly
    IUM2WTL,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 3 year, 75% LTV, Combined bank and building society, Monthly
    IUMBV37,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 5 year, 60% LTV, Monthly
    IUMZO27,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 	5 year, 75% LTV, Combined bank and building society, Monthly
    IUMBV42,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 5 year, 90% LTV, Monthly
    IUMZO28,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 5 year, 95% LTV, Combined bank and building society, Monthly
    IUM5WTL,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, 10 year, 75% LTV, Combined bank and building society, Monthly
    IUMBV45,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, Buy-to-let 2 year, 60% LTV, Monthly
    IUMZO29,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, Buy-to-let 2 year, 75% LTV, Monthly
    IUMZID4,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, Buy-to-let 5 year, 60% LTV, Monthly
    IUMZO2A,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed, Buy-to-let 5 year, 75% LTV, Monthly
    IUMZO2B,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed term variable rate, 2 year, 75% LTV, Combined bank and building society, Monthly
    IUMBV48,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed term variable rate, 2 year, 90% LTV, Combined bank and building society, Monthly
    IUMB479,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Fixed term variable rate, 2 year, 95% LTV, Combined bank and building society, Monthly
    IUM2WDT,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Revert-to-rate, Combined bank and building society, Monthly
    IUMTLMV,
    /// Quoted household interest rates, Secured lending (mortgage) rates, Lifetime Tracker, Combined bank and building society, Monthly
    IUMBV24,
    /// Quoted household interest rates, Unsecured lending rates, Personal loan, £3,000 Combined bank and building society, Monthly
    IUMZO2C,
    /// Quoted household interest rates, Unsecured lending rates, Personal loan, £5,000 Combined bank and building society, Monthly
    IUMBX67,
    /// Quoted household interest rates, Unsecured lending rates, Personal loan, £10,000 Combined bank and building society, Monthly
    IUMHPTL,
    /// Quoted household interest rates, Unsecured lending rates, Personal loan, £25,000 Combined bank and building society, Monthly
    IUMZO2D,
    /// Quoted household interest rates, Unsecured lending rates, Credit card, Combined bank and building society, Representative card, Monthly
    IUMCCTL,
    /// Quoted household interest rates, Unsecured lending rates, Credit card, Combined bank and building society, 0% purchase period, Monthly
    IUMZO2E,
    /// Quoted household interest rates, Unsecured lending rates, Credit card, Combined bank and building society, 0% balance transfer, Monthly
    IUMZO2F,
    /// Quoted household interest rates, Unsecured lending rates, Credit card, Combined bank and building society, Lowest APR, Monthly
    IUMZO2G,
    /// Quoted household interest rates, Unsecured lending rates, Overdraft, Combined bank and building society, Monthly
    IUMODTL,
    /// Quoted household interest rates, Deposit rates, Instant access savings, Including unconditional bonuses, Combined bank and building society, Monthly
    IUMB6VJ,
    /// Quoted household interest rates, Deposit rates, Instant access savings, Excluding unconditional bonuses, Combined bank and building society, Monthly
    IUMB6VK,
    /// Quoted household interest rates, Deposit rates, Instant access savings, Branch-based (excluding bonuses), Combined bank and building society, Monthly
    IUMTHAK,
    /// Quoted household interest rates, Deposit rates, Cash ISA, Variable rate, Including unconditional bonuses, Combined bank and building society, Monthly
    IUMB6VL,
    /// Quoted household interest rates, Deposit rates, Cash ISA, Variable rate, Excluding unconditional bonuses, Combined bank and building society, Monthly
    IUMB6VM,
    /// Quoted household interest rates, Deposit rates, Cash ISA, Variable rate, Branch-based excluding bonuses, Combined bank and building society, Monthly
    IUMWTIS,
    /// Quoted household interest rates, Deposit rates, Cash ISA, Fixed rate 1 year, Combined bank and building society, Monthly
    IUMB6VN,
    /// Quoted household interest rates, Deposit rates, Cash ISA, Fixed rate 2 year, Monthly
    IUMZID2,
    /// Quoted household interest rates, Deposit rates, Fixed rate bonds, 1 year, Combined bank and building society, Monthly
    IUMWTFA,
    /// Quoted household interest rates, Deposit rates, Fixed rate bonds, 2 year, Combined bank and building society, Monthly
    IUMB6RH,
    /// Quoted household interest rates, Deposit rates, Fixed rate bonds, 3 year, Monthly
    IUMB6RI,
    /// Quoted household interest rates, Deposit rates, Time (notice accounts), Combined bank and building society, Monthly
    IUMWTTA,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Public Sector, Interest bearing sight, Monthly
    CFMHSCP,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Public Sector, Time, Monthly
    CFMHSCQ,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Central and Local Government, Interest bearing sight, Monthly
    CFMBI22,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Central and Local Government, Time, Monthly
    CFMBI23,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Public Corporations, Interest bearing sight, Monthly
    CFMBJ59,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Public Corporations, Time, Monthly
    CFMBJ62,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Banks (until December 2009), Interest bearing sight, Monthly
    CFMBI28,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Banks (until December 2009), Time, Monthly
    CFMBI29,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Building Societies (until December 2009), Interest bearing sight, Monthly
    CFMHSDM,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Building Societies (until December 2009), Time, Monthly
    CFMHSDN,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Monetary financial institutions, Interest bearing sight, Monthly
    CFMB2HW,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Monetary financial institutions, Time, Monthly
    CFMB2HX,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Other Financial Corporations, Interest bearing sight, Monthly
    CFMHSCR,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Other Financial Corporations, Time, Monthly
    CFMHSCS,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Private Non-Financial Corporations, Interest bearing sight, Monthly
    CFMHSCT,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Private Non-Financial Corporations, Time, Redeemable at notice, Monthly
    CFMBI35,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Private Non-Financial Corporations, Time, Total, Monthly
    CFMHSCU,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Households, Interest bearing sight, Monthly
    CFMHSCV,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Households, Time, Redeemable at notice, Monthly
    CFMBJ65,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Households, Time, Total, Monthly
    CFMHSCW,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Non-Profit Institutions, Interest bearing sight, Monthly
    CFMHSCX,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Non-Profit Institutions, Time, Redeemable at notic, Monthly
    CFMBJ67,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Non-Profit Institutions, Time, Total, Monthly
    CFMHSCY,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Time, Total, Monthly
    CFMZ6IW,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Interest bearing sight, Total, Monthly
    CFMZ6IQ,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Interest bearing sight, Total of which current accounts, Monthly
    CFMZ6IU,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Interest bearing sight, Monthly
    CFMZ6LL,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Time, Redeemable at notice, Monthly
    CFMZ6K4,
    /// Effective Interest Rates, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Time, Total, Monthly
    CFMZ6LK,
    /// Effective Interest Rates, New business rates for sterling fixed rate, fixed maturity time deposits placed with UK monetary financial institutions (excl. central bank) in the month, Public Corporations, Monthly
    CFMBJ69,
    /// Effective Interest Rates, New business rates for sterling fixed rate, fixed maturity time deposits placed with UK monetary financial institutions (excl. central bank) in the month, Private Non-Financial Corporations, Fixed maturity, Total, Monthly
    CFMBJ72,
    /// Effective Interest Rates, New business rates for sterling fixed rate, fixed maturity time deposits placed with UK monetary financial institutions (excl. central bank) in the month, Households	Fixed maturity, Total, Monthly
    CFMBJ74,
    /// Effective Interest Rates, New business rates for sterling fixed rate, fixed maturity time deposits placed with UK monetary financial institutions (excl. central bank) in the month, Households	Fixed maturity, Total of which fixed rate bonds, Monthly
    CFMBX2N,
    /// Effective Interest Rates, New business rates for sterling fixed rate, fixed maturity time deposits placed with UK monetary financial institutions (excl. central bank) in the month, Non-profit institutions, Monthly
    CFMBI87,
    /// Effective Interest Rates, New business rates for sterling fixed rate, fixed maturity time deposits placed with UK monetary financial institutions (excl. central bank) in the month, Individuals and individual trusts, Time, Fixed maturity, Total, Monthly
    CFMZ6IH,
    /// Effective Interest Rates, New business rates for sterling fixed rate, fixed maturity time deposits placed with UK monetary financial institutions (excl. central bank) in the month, Unincorporated Businesses, Time, Fixed maturity, Total, Monthly
    CFMZ6JE,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Public Sector, Monthly
    CFMHSCZ,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Central & Local Government, Monthly
    CFMBI49,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Public Corporations, Loans, Monthly
    CFMBJ75,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Public Corporations, Overdrafts, Monthly
    CFMBI52,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Banks (until December 2009), Monthly
    CFMBI57,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Building Societies (until December 2009), Monthly
    CFMHSDO,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Monetary financial institutions, Monthly
    CFMB2HY,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Other Financial Corporations, Monthly
    CFMHSDA,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Private non-Financial Corporations, Loans, Floating rate, Monthly
    CFMBI58,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Private non-Financial Corporations, Loans, Fixed rate, Total, Monthly
    CFMHSDC,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Private non-Financial Corporations, Overdrafts, Monthly
    CFMHSDB,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Personal Loans, Floating rate, Monthly
    CFMBI69,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Personal Loans, Fixed Rate, Total, Monthly
    CFMHSDI,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Credit Cards, Interest bearing balances, Monthly
    CFMHSDG,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Credit Cards, All balances, Monthly
    CFMHSDP,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Credit Cards, Overdrafts, Monthly
    CFMHSDH,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Secured on dwellings, Floating rate, Monthly
    CFMBI64,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Secured on dwellings, Floating rate of which SVR, Monthly
    CFMBX2D,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Secured on dwellings, Floating rate of which lifetime tracker, Monthly
    CFMBX2E,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Secured on dwellings, Fixed Rate, Total, Monthly
    CFMHSDE,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Bridging Loans, Monthly
    CFMHSDD,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Non-Profit Institutions, Loans, Monthly
    CFMHSDK,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Non-Profit Institutions, Overdraft, Monthly
    CFMHSDJ,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Credit cards, Interest bearing balances, Total, Monthly
    CFMZ6IR,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Credit cards, All balances, Total, Monthly
    CFMZ6IS,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Secured on dwellings, Floating rate, Total, Monthly
    CFMZ6K8,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Secured on dwellings, Fixed rate, Total, Monthly
    CFMZ6KA,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Secured on dwellings, Total, Monthly
    CFMZ6K6,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Overdrafts, Interest charging, Monthly
    CFMZJ4A,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Overdrafts, Interest and fee charging, Monthly
    CFMZ6KM,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Other loans, Floating-rate, Monthly
    CFMZ6KQ,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Other loans, Fixed-rate, Monthly
    CFMZ6LI,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Individuals and individual trusts, Other loans, Total, Monthly
    CFMZ6KO,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Overdrafts, Monthly
    CFMZ6KX,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Secured loans, Floating-rate, Monthly
    CFMZ6L3,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Secured loans, Fixed-rate, Monthly
    CFMZ6L5,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Secured loans, Total, Monthly
    CFMZ6KZ,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Other loans, Floating-rate, Monthly
    CFMZ6LU,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Other loans, Fixed-rate, Monthly
    CFMZ6LE,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Unincorporated Businesses, Other loans, Total, Monthly
    CFMZ6LT,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Small and medium sized PNFCs, Other loans, Floating-rate, Bank Rate linked, Total, Monthly
    CFMZ6LR,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Small and medium sized PNFCs, Other loans, Floating-rate, SONIA linked, Total, Monthly
    CFMZ6HU,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Small and medium sized PNFCs, Other loans, Floating-rate, Total, Monthly
    CFMZ6LQ,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Small and medium sized PNFCs, Other loans, Fixed-rate, Total, Monthly
    CFMZ6I6,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Small and medium sized PNFCs, Other loans, Total, Monthly
    CFMZ6LN,
    /// Effective Interest Rates, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Small and medium sized PNFCs, Overdrafts, Monthly
    CFMZ6IF,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Public Corporations, Monthly
    CFMBJ79,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Private Non-financial Corporations, Loans by rate type, Floating rate, Monthly
    CFMBJ83,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Private Non-financial Corporations, Loans by rate type, Fixed rate, Total, Monthly
    CFMBJ84,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Private Non-financial Corporations, Loans by original size, Total, Monthly
    CFMBJ82,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Personal Loans, Floating Rate, Monthly
    CFMBJ47,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Personal Loans, Fixed Rate, Total, Monthly
    CFMBJ94,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Personal Loans, Total, Monthly
    CFMBJ93,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Secured on Dwellings, Floating Rate, Monthly
    CFMBJ39,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Secured on Dwellings, Fixed Rate, Total, Monthly
    CFMBJ96,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Secured on Dwellings, Total, Monthly
    CFMBJ95,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Bridging Loans, Monthly
    CFMBJ38,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Non-Profit Institutions, Monthly
    CFMBJ97,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Individuals and individual trusts, Secured on dwellings, Fixed Rate, Total, Monthly
    CFMZ6JV,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Individuals and individual trusts, Secured on dwellings, Floating Rate, Total, Monthly
    CFMZ6JO,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Individuals and individual trusts, Secured on dwellings, Floating Rate of which lifetime tracker, Monthly
    CFMZ6JT,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Individuals and individual trusts, Secured on dwellings, Total, Monthly
    CFMZ6JM,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Individuals and individual trusts, Other loans, Floating-rate, Monthly
    CFMZ6K7,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Individuals and individual trusts, Other loans, Fixed-rate, Monthly
    CFMZ6K9,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Individuals and individual trusts, Other loans, Total, Monthly
    CFMZ6K5,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Small and medium sized PNFCs, Other loans, Floating-rate, Bank Rate linked, Total, Monthly
    CFMZJ3M,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Small and medium sized PNFCs, Other loans, Floating-rate, SONIA linked, Total, Monthly
    CFMZJ3Q,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Small and medium sized PNFCs, Other loans, Floating-rate, Total, Monthly
    CFMZJ3L,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Small and medium sized PNFCs, Other loans, Fixed-rate, Total, Monthly
    CFMZJ3U,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Small and medium sized PNFCs, Other loans, Total, Monthly
    CFMZ6LD,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Unincorporated Businesses, Secured loans, Floating-rate, Monthly
    CFMZ6KJ,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Unincorporated Businesses, Secured loans, Fixed-rate, Total, Monthly
    CFMZ6KL,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Unincorporated Businesses, Secured loans, Total, Monthly
    CFMZ6KH,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Unincorporated Businesses, Other loans, Floating-rate, Monthly
    CFMZ6KY,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Unincorporated Businesses, Other loans, Fixed-rate, Total, Monthly
    CFMZ6L2,
    /// Effective Interest Rates, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Unincorporated Businesses, Other loans, Total, Monthly
    CFMZ6KW,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling time deposits with UK monetary financial institutions (excl. Central bank), Private Non-Financial Corporations, Redeemable at notice, Quarterly
    CFQBK2B,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling time deposits with UK monetary financial institutions (excl. Central bank), Private Non-Financial Corporations, Fixed maturity, Total fixed, Quarterly
    CFQB9KZ,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling time deposits with UK monetary financial institutions (excl. Central bank), Households, Redeemable at notice, Quarterly
    CFQB9KV,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling time deposits with UK monetary financial institutions (excl. Central bank), Households, Fixed maturity, Total fixed, Quarterly
    CFQB9KU,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling time deposits with UK monetary financial institutions (excl. Central bank), Individuals and individual trusts, Redeemable at notice, Quarterly
    CFQZJ3Y,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling time deposits with UK monetary financial institutions (excl. Central bank), Individuals and individual trusts, Fixed maturity, Total fixed, Quarterly
    CFQZJ3Z,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Private Non-Financial Corporations, Loans, Floating rate, Quarterly
    CFQB3OZ,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Private Non-Financial Corporations, Loans, Fixed rate, Total fixed, Quarterly
    CFQB3RY,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Households, Unsecured loans, Floating rate, Quarterly
    CFQB3RU,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Households, Unsecured loans, Fixed rate, Total fixed, Quarterly
    CFQB3RT,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Households, Secured on dwellings, Floating rate, Quarterly
    CFQBK2N,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Households, Secured on dwellings, Fixed rate, Total fixed, Quarterly
    CFQBK2M,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Individuals and individual trusts, Secured on dwellings, Floating rate, Quarterly
    CFQZJ48,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Individuals and individual trusts, Secured on dwellings, Fixed rate, Total fixed, Quarterly
    CFQZJ49,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Individuals and individual trusts, Other loans, Floating rate, Quarterly
    CFQZJ4E,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), Individuals and individual trusts, Other loans, Fixed rate, Total fixed, Quarterly
    CFQZJ4F,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), SMEs, Floating rate, Quarterly
    CFQZJ4J,
    /// Effective Interest Rates, Distribution of Balances, Outstanding sterling loans by UK monetary financial institutions (excl. Central bank), SMEs, Fixed rate, Total fixed, Quarterly
    CFQZJ4K,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Private Non-Financial Corporations, Loans, Floating rate, Quarterly
    CFQB4VP,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Private Non-Financial Corporations, Loans, Fixed rate, Total fixed, Quarterly
    CFQB4VO,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Households, Unsecured loans, Floating rate, Quarterly
    CFQB4VK,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Households, Unsecured loans, Fixed rate, Total fixed, Quarterly
    CFQB4VJ,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Households, Secured on dwellings, Floating rate, Quarterly
    CFQB4VF,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Households, Secured on dwellings, Fixed rate, Total fixed, Quarterly
    CFQB4VE,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Individuals and individual trusts, Secured on dwellings, Floating rate, Quarterly
    CFQZJ4U,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Individuals and individual trusts, Secured on dwellings, Fixed rate, Total fixed, Quarterly
    CFQZJ4V,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Individuals and individual trusts, Other loans, Floating rate, Quarterly
    CFQZJ54,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, Individuals and individual trusts, Other loans, Fixed rate, Total fixed, Quarterly
    CFQZJ55,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, SMEs, Floating rate, Quarterly
    CFQZJ59,
    /// Effective Interest Rates, Distribution of Balances, New sterling loans by UK monetary financial institutions (excl. Central bank) in the month, SMEs, Fixed rate, Total fixed, Quarterly
    CFQZJ5A,
    /// Effective Interest Rates, Additional series published by the ECB, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Households, Time, Redeemable at notice, up to 3 months, Monthly
    CFMB2CT,
    /// Effective Interest Rates, Additional series published by the ECB, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Households, Time, Redeemable at notice, over 3 months, Monthly
    CFMB2CU,
    /// Effective Interest Rates, Additional series published by the ECB, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Households, Time, Fixed Maturity, up to 2 years, Monthly
    CFMB2CP,
    /// Effective Interest Rates, Additional series published by the ECB, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Private Non-Financial Corporations, Time, Fixed Maturity, up to 2 years, Monthly
    CFMB2CQ,
    /// Effective Interest Rates, Additional series published by the ECB, Outstanding sterling deposits with UK monetary financial institutions (excl. central bank), Private Non-Financial Corporations, Time, Fixed Maturity, over 2 years, Monthly
    CFMB2CR,
    /// Effective Interest Rates, Additional series published by the ECB, New business rates for sterling fixed rate, fixed maturity time deposits placed with UK monetary financial institutions (excl. central bank) in the month, Private Non-Financial Corporations, Time, Fixed Maturity, over 1 year up to 2 years, Monthly
    CFMB2CV,
    /// Effective Interest Rates, Additional series published by the ECB, Outstanding sterling loans by UK monetary financial institutions (excl. central bank), Households, Secured on dwellings, Fixed Maturity, over 5 years, Monthly
    CFMB2CS,
    /// Effective Interest Rates, Additional series published by the ECB, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Personal Loans, Floating and Fixed Rate, up to 1 year, Monthly
    CFMB2CW,
    /// Effective Interest Rates, Additional series published by the ECB, New business rates for sterling lending undertaken by UK monetary financial institutions (excl. central bank) in the month, Households, Secured on Dwellings, Floating and Fixed Rate, up to 1 year, Monthly
    CFMB2CX,
}

impl fmt::Display for SeriesCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeriesCode::IUDSOIA => write!(f, "IUDSOIA"),
            SeriesCode::XUDLCDS => write!(f, "XUDLCDS"),
            SeriesCode::XUDLDKS => write!(f, "XUDLDKS"),
            SeriesCode::XUDLERS => write!(f, "XUDLERS"),
            SeriesCode::XUDLJYS => write!(f, "XUDLJYS"),
            SeriesCode::XUDLNKS => write!(f, "XUDLNKS"),
            SeriesCode::XUDLSFS => write!(f, "XUDLSFS"),
            SeriesCode::XUDLSGS => write!(f, "XUDLSGS"),
            SeriesCode::XUDLSKS => write!(f, "XUDLSKS"),
            SeriesCode::XUDLUSS => write!(f, "XUDLUSS"),
            SeriesCode::IUDBEDR => write!(f, "IUDBEDR"),
            SeriesCode::IUDAMIH => write!(f, "IUDAMIH"),
            SeriesCode::XUDLDF1 => write!(f, "XUDLDF1"),
            SeriesCode::XUDLDF3 => write!(f, "XUDLDF3"),
            SeriesCode::XUDLDF6 => write!(f, "XUDLDF6"),
            SeriesCode::XUDLDFY => write!(f, "XUDLDFY"),
            SeriesCode::XUDLDS1 => write!(f, "XUDLDS1"),
            SeriesCode::XUDLDS3 => write!(f, "XUDLDS3"),
            SeriesCode::XUDLDS6 => write!(f, "XUDLDS6"),
            SeriesCode::XUDLDSY => write!(f, "XUDLDSY"),
            SeriesCode::XUDLGPS => write!(f, "XUDLGPS"),
            SeriesCode::XUDLGPD => write!(f, "XUDLGPD"),
            SeriesCode::IUDSNPY => write!(f, "IUDSNPY"),
            SeriesCode::IUDMNPY => write!(f, "IUDMNPY"),
            SeriesCode::IUDLNPY => write!(f, "IUDLNPY"),
            SeriesCode::IUDSIZC => write!(f, "IUDSIZC"),
            SeriesCode::IUDMIZC => write!(f, "IUDMIZC"),
            SeriesCode::IUDLIZC => write!(f, "IUDLIZC"),
            SeriesCode::IUDSIIF => write!(f, "IUDSIIF"),
            SeriesCode::IUDMIIF => write!(f, "IUDMIIF"),
            SeriesCode::IUDLIIF => write!(f, "IUDLIIF"),
            SeriesCode::IUDWRLN => write!(f, "IUDWRLN"),
            SeriesCode::IUDAJUR => write!(f, "IUDAJUR"),
            SeriesCode::IUDEBEN => write!(f, "IUDEBEN"),
            SeriesCode::IUDAJLT => write!(f, "IUDAJLT"),
            SeriesCode::IUDBK58 => write!(f, "IUDBK58"),
            SeriesCode::IUDAJLW => write!(f, "IUDAJLW"),
            SeriesCode::IUMZICQ => write!(f, "IUMZICQ"),
            SeriesCode::IUMBV34 => write!(f, "IUMBV34"),
            SeriesCode::IUMZICR => write!(f, "IUMZICR"),
            SeriesCode::IUMB482 => write!(f, "IUMB482"),
            SeriesCode::IUM2WTL => write!(f, "IUM2WTL"),
            SeriesCode::IUMBV37 => write!(f, "IUMBV37"),
            SeriesCode::IUMZO27 => write!(f, "IUMZO27"),
            SeriesCode::IUMBV42 => write!(f, "IUMBV42"),
            SeriesCode::IUMZO28 => write!(f, "IUMZO28"),
            SeriesCode::IUM5WTL => write!(f, "IUM5WTL"),
            SeriesCode::IUMBV45 => write!(f, "IUMBV45"),
            SeriesCode::IUMZO29 => write!(f, "IUMZO29"),
            SeriesCode::IUMZID4 => write!(f, "IUMZID4"),
            SeriesCode::IUMZO2A => write!(f, "IUMZO2A"),
            SeriesCode::IUMZO2B => write!(f, "IUMZO2B"),
            SeriesCode::IUMBV48 => write!(f, "IUMBV48"),
            SeriesCode::IUMB479 => write!(f, "IUMB479"),
            SeriesCode::IUM2WDT => write!(f, "IUM2WDT"),
            SeriesCode::IUMTLMV => write!(f, "IUMTLMV"),
            SeriesCode::IUMBV24 => write!(f, "IUMBV24"),
            SeriesCode::IUMZO2C => write!(f, "IUMZO2C"),
            SeriesCode::IUMBX67 => write!(f, "IUMBX67"),
            SeriesCode::IUMHPTL => write!(f, "IUMHPTL"),
            SeriesCode::IUMZO2D => write!(f, "IUMZO2D"),
            SeriesCode::IUMCCTL => write!(f, "IUMCCTL"),
            SeriesCode::IUMZO2E => write!(f, "IUMZO2E"),
            SeriesCode::IUMZO2F => write!(f, "IUMZO2F"),
            SeriesCode::IUMZO2G => write!(f, "IUMZO2G"),
            SeriesCode::IUMODTL => write!(f, "IUMODTL"),
            SeriesCode::IUMB6VJ => write!(f, "IUMB6VJ"),
            SeriesCode::IUMB6VK => write!(f, "IUMB6VK"),
            SeriesCode::IUMTHAK => write!(f, "IUMTHAK"),
            SeriesCode::IUMB6VL => write!(f, "IUMB6VL"),
            SeriesCode::IUMB6VM => write!(f, "IUMB6VM"),
            SeriesCode::IUMWTIS => write!(f, "IUMWTIS"),
            SeriesCode::IUMB6VN => write!(f, "IUMB6VN"),
            SeriesCode::IUMZID2 => write!(f, "IUMZID2"),
            SeriesCode::IUMWTFA => write!(f, "IUMWTFA"),
            SeriesCode::IUMB6RH => write!(f, "IUMB6RH"),
            SeriesCode::IUMB6RI => write!(f, "IUMB6RI"),
            SeriesCode::IUMWTTA => write!(f, "IUMWTTA"),
            SeriesCode::CFMHSCP => write!(f, "CFMHSCP"),
            SeriesCode::CFMHSCQ => write!(f, "CFMHSCQ"),
            SeriesCode::CFMBI22 => write!(f, "CFMBI22"),
            SeriesCode::CFMBI23 => write!(f, "CFMBI23"),
            SeriesCode::CFMBJ59 => write!(f, "CFMBJ59"),
            SeriesCode::CFMBJ62 => write!(f, "CFMBJ62"),
            SeriesCode::CFMBI28 => write!(f, "CFMBI28"),
            SeriesCode::CFMBI29 => write!(f, "CFMBI29"),
            SeriesCode::CFMHSDM => write!(f, "CFMHSDM"),
            SeriesCode::CFMHSDN => write!(f, "CFMHSDN"),
            SeriesCode::CFMB2HW => write!(f, "CFMB2HW"),
            SeriesCode::CFMB2HX => write!(f, "CFMB2HX"),
            SeriesCode::CFMHSCR => write!(f, "CFMHSCR"),
            SeriesCode::CFMHSCS => write!(f, "CFMHSCS"),
            SeriesCode::CFMHSCT => write!(f, "CFMHSCT"),
            SeriesCode::CFMBI35 => write!(f, "CFMBI35"),
            SeriesCode::CFMHSCU => write!(f, "CFMHSCU"),
            SeriesCode::CFMHSCV => write!(f, "CFMHSCV"),
            SeriesCode::CFMBJ65 => write!(f, "CFMBJ65"),
            SeriesCode::CFMHSCW => write!(f, "CFMHSCW"),
            SeriesCode::CFMHSCX => write!(f, "CFMHSCX"),
            SeriesCode::CFMBJ67 => write!(f, "CFMBJ67"),
            SeriesCode::CFMHSCY => write!(f, "CFMHSCY"),
            SeriesCode::CFMZ6IW => write!(f, "CFMZ6IW"),
            SeriesCode::CFMZ6IQ => write!(f, "CFMZ6IQ"),
            SeriesCode::CFMZ6IU => write!(f, "CFMZ6IU"),
            SeriesCode::CFMZ6LL => write!(f, "CFMZ6LL"),
            SeriesCode::CFMZ6K4 => write!(f, "CFMZ6K4"),
            SeriesCode::CFMZ6LK => write!(f, "CFMZ6LK"),
            SeriesCode::CFMBJ69 => write!(f, "CFMBJ69"),
            SeriesCode::CFMBJ72 => write!(f, "CFMBJ72"),
            SeriesCode::CFMBJ74 => write!(f, "CFMBJ74"),
            SeriesCode::CFMBX2N => write!(f, "CFMBX2N"),
            SeriesCode::CFMBI87 => write!(f, "CFMBI87"),
            SeriesCode::CFMZ6IH => write!(f, "CFMZ6IH"),
            SeriesCode::CFMZ6JE => write!(f, "CFMZ6JE"),
            SeriesCode::CFMHSCZ => write!(f, "CFMHSCZ"),
            SeriesCode::CFMBI49 => write!(f, "CFMBI49"),
            SeriesCode::CFMBJ75 => write!(f, "CFMBJ75"),
            SeriesCode::CFMBI52 => write!(f, "CFMBI52"),
            SeriesCode::CFMBI57 => write!(f, "CFMBI57"),
            SeriesCode::CFMHSDO => write!(f, "CFMHSDO"),
            SeriesCode::CFMB2HY => write!(f, "CFMB2HY"),
            SeriesCode::CFMHSDA => write!(f, "CFMHSDA"),
            SeriesCode::CFMBI58 => write!(f, "CFMBI58"),
            SeriesCode::CFMHSDC => write!(f, "CFMHSDC"),
            SeriesCode::CFMHSDB => write!(f, "CFMHSDB"),
            SeriesCode::CFMBI69 => write!(f, "CFMBI69"),
            SeriesCode::CFMHSDI => write!(f, "CFMHSDI"),
            SeriesCode::CFMHSDG => write!(f, "CFMHSDG"),
            SeriesCode::CFMHSDP => write!(f, "CFMHSDP"),
            SeriesCode::CFMHSDH => write!(f, "CFMHSDH"),
            SeriesCode::CFMBI64 => write!(f, "CFMBI64"),
            SeriesCode::CFMBX2D => write!(f, "CFMBX2D"),
            SeriesCode::CFMBX2E => write!(f, "CFMBX2E"),
            SeriesCode::CFMHSDE => write!(f, "CFMHSDE"),
            SeriesCode::CFMHSDD => write!(f, "CFMHSDD"),
            SeriesCode::CFMHSDK => write!(f, "CFMHSDK"),
            SeriesCode::CFMHSDJ => write!(f, "CFMHSDJ"),
            SeriesCode::CFMZ6IR => write!(f, "CFMZ6IR"),
            SeriesCode::CFMZ6IS => write!(f, "CFMZ6IS"),
            SeriesCode::CFMZ6K8 => write!(f, "CFMZ6K8"),
            SeriesCode::CFMZ6KA => write!(f, "CFMZ6KA"),
            SeriesCode::CFMZ6K6 => write!(f, "CFMZ6K6"),
            SeriesCode::CFMZJ4A => write!(f, "CFMZJ4A"),
            SeriesCode::CFMZ6KM => write!(f, "CFMZ6KM"),
            SeriesCode::CFMZ6KQ => write!(f, "CFMZ6KQ"),
            SeriesCode::CFMZ6LI => write!(f, "CFMZ6LI"),
            SeriesCode::CFMZ6KO => write!(f, "CFMZ6KO"),
            SeriesCode::CFMZ6KX => write!(f, "CFMZ6KX"),
            SeriesCode::CFMZ6L3 => write!(f, "CFMZ6L3"),
            SeriesCode::CFMZ6L5 => write!(f, "CFMZ6L5"),
            SeriesCode::CFMZ6KZ => write!(f, "CFMZ6KZ"),
            SeriesCode::CFMZ6LU => write!(f, "CFMZ6LU"),
            SeriesCode::CFMZ6LE => write!(f, "CFMZ6LE"),
            SeriesCode::CFMZ6LT => write!(f, "CFMZ6LT"),
            SeriesCode::CFMZ6LR => write!(f, "CFMZ6LR"),
            SeriesCode::CFMZ6HU => write!(f, "CFMZ6HU"),
            SeriesCode::CFMZ6LQ => write!(f, "CFMZ6LQ"),
            SeriesCode::CFMZ6I6 => write!(f, "CFMZ6I6"),
            SeriesCode::CFMZ6LN => write!(f, "CFMZ6LN"),
            SeriesCode::CFMZ6IF => write!(f, "CFMZ6IF"),
            SeriesCode::CFMBJ79 => write!(f, "CFMBJ79"),
            SeriesCode::CFMBJ83 => write!(f, "CFMBJ83"),
            SeriesCode::CFMBJ84 => write!(f, "CFMBJ84"),
            SeriesCode::CFMBJ82 => write!(f, "CFMBJ82"),
            SeriesCode::CFMBJ47 => write!(f, "CFMBJ47"),
            SeriesCode::CFMBJ94 => write!(f, "CFMBJ94"),
            SeriesCode::CFMBJ93 => write!(f, "CFMBJ93"),
            SeriesCode::CFMBJ39 => write!(f, "CFMBJ39"),
            SeriesCode::CFMBJ96 => write!(f, "CFMBJ96"),
            SeriesCode::CFMBJ95 => write!(f, "CFMBJ95"),
            SeriesCode::CFMBJ38 => write!(f, "CFMBJ38"),
            SeriesCode::CFMBJ97 => write!(f, "CFMBJ97"),
            SeriesCode::CFMZ6JV => write!(f, "CFMZ6JV"),
            SeriesCode::CFMZ6JO => write!(f, "CFMZ6JO"),
            SeriesCode::CFMZ6JT => write!(f, "CFMZ6JT"),
            SeriesCode::CFMZ6JM => write!(f, "CFMZ6JM"),
            SeriesCode::CFMZ6K7 => write!(f, "CFMZ6K7"),
            SeriesCode::CFMZ6K9 => write!(f, "CFMZ6K9"),
            SeriesCode::CFMZ6K5 => write!(f, "CFMZ6K5"),
            SeriesCode::CFMZJ3M => write!(f, "CFMZJ3M"),
            SeriesCode::CFMZJ3Q => write!(f, "CFMZJ3Q"),
            SeriesCode::CFMZJ3L => write!(f, "CFMZJ3L"),
            SeriesCode::CFMZJ3U => write!(f, "CFMZJ3U"),
            SeriesCode::CFMZ6LD => write!(f, "CFMZ6LD"),
            SeriesCode::CFMZ6KJ => write!(f, "CFMZ6KJ"),
            SeriesCode::CFMZ6KL => write!(f, "CFMZ6KL"),
            SeriesCode::CFMZ6KH => write!(f, "CFMZ6KH"),
            SeriesCode::CFMZ6KY => write!(f, "CFMZ6KY"),
            SeriesCode::CFMZ6L2 => write!(f, "CFMZ6L2"),
            SeriesCode::CFMZ6KW => write!(f, "CFMZ6KW"),
            SeriesCode::CFQBK2B => write!(f, "CFQBK2B"),
            SeriesCode::CFQB9KZ => write!(f, "CFQB9KZ"),
            SeriesCode::CFQB9KV => write!(f, "CFQB9KV"),
            SeriesCode::CFQB9KU => write!(f, "CFQB9KU"),
            SeriesCode::CFQZJ3Y => write!(f, "CFQZJ3Y"),
            SeriesCode::CFQZJ3Z => write!(f, "CFQZJ3Z"),
            SeriesCode::CFQB3OZ => write!(f, "CFQB3OZ"),
            SeriesCode::CFQB3RY => write!(f, "CFQB3RY"),
            SeriesCode::CFQB3RU => write!(f, "CFQB3RU"),
            SeriesCode::CFQB3RT => write!(f, "CFQB3RT"),
            SeriesCode::CFQBK2N => write!(f, "CFQBK2N"),
            SeriesCode::CFQBK2M => write!(f, "CFQBK2M"),
            SeriesCode::CFQZJ48 => write!(f, "CFQZJ48"),
            SeriesCode::CFQZJ49 => write!(f, "CFQZJ49"),
            SeriesCode::CFQZJ4E => write!(f, "CFQZJ4E"),
            SeriesCode::CFQZJ4F => write!(f, "CFQZJ4F"),
            SeriesCode::CFQZJ4J => write!(f, "CFQZJ4J"),
            SeriesCode::CFQZJ4K => write!(f, "CFQZJ4K"),
            SeriesCode::CFQB4VP => write!(f, "CFQB4VP"),
            SeriesCode::CFQB4VO => write!(f, "CFQB4VO"),
            SeriesCode::CFQB4VK => write!(f, "CFQB4VK"),
            SeriesCode::CFQB4VJ => write!(f, "CFQB4VJ"),
            SeriesCode::CFQB4VF => write!(f, "CFQB4VF"),
            SeriesCode::CFQB4VE => write!(f, "CFQB4VE"),
            SeriesCode::CFQZJ4U => write!(f, "CFQZJ4U"),
            SeriesCode::CFQZJ4V => write!(f, "CFQZJ4V"),
            SeriesCode::CFQZJ54 => write!(f, "CFQZJ54"),
            SeriesCode::CFQZJ55 => write!(f, "CFQZJ55"),
            SeriesCode::CFQZJ59 => write!(f, "CFQZJ59"),
            SeriesCode::CFQZJ5A => write!(f, "CFQZJ5A"),
            SeriesCode::CFMB2CT => write!(f, "CFMB2CT"),
            SeriesCode::CFMB2CU => write!(f, "CFMB2CU"),
            SeriesCode::CFMB2CP => write!(f, "CFMB2CP"),
            SeriesCode::CFMB2CQ => write!(f, "CFMB2CQ"),
            SeriesCode::CFMB2CR => write!(f, "CFMB2CR"),
            SeriesCode::CFMB2CV => write!(f, "CFMB2CV"),
            SeriesCode::CFMB2CS => write!(f, "CFMB2CS"),
            SeriesCode::CFMB2CW => write!(f, "CFMB2CW"),
            SeriesCode::CFMB2CX => write!(f, "CFMB2CX"),
        }
    }
}