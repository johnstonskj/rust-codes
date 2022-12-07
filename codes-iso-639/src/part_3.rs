/*!
This module provides an implementation of ISO 639-3, Codes for the
representation of names of languages — Part 3: Alpha-3 code for comprehensive
coverage of languages.
 */

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A language identifier (also known as a language code or language code
/// element) represents one or more language names, all of which designate the
/// same specific language. The ultimate objects of identification are
/// languages themselves; language names are the formal means by which the
/// languages denoted by language identifiers are designated.
///
///Languages are not static objects; there is variation temporally, spacially,
/// and socially; every language corresponds to some range of variation in
/// linguistic expression. In this part of ISO 639, then, a language
/// identifier denotes some range of language varieties. The scope of
/// languages in Part 3 of ISO 639 is either individual language or
/// macrolanguage. Parts 2 and 5 of ISO 639 also contain codes whose scope is
/// collection of languages.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LanguageScope {
    /// Individual languages
    ///
    /// In Part 3 of ISO 639, most identifiers are assumed to denote distinct
    /// individual languages. Furthermore, it is a goal for this part of ISO
    /// 639 to provide an identifier for every distinct human language that
    /// has been documented, whether living, extinct, or constructed, and
    /// whether its modality is spoken, written or signed.
    ///
    Individual,
    /// Macrolanguages
    ///
    /// Other parts of ISO 639 have included identifiers designated as
    /// individual language identifiers that correspond in a one-to-many
    /// manner with individual language identifiers in this part of ISO 639.
    /// For instance, this part of ISO 639 contains over 30 identifiers
    /// designated as individual language identifiers for distinct varieties
    /// of Arabic, while ISO 639-1 and ISO 639-2 each contain only one
    /// identifier for Arabic, "ar" and "ara" respectively, which are
    /// designated as individual language identifiers in those parts of ISO
    /// 639. It is assumed here that the single identifiers for Arabic in
    /// parts 1 and 2 of ISO 639 correspond to the many identifiers
    /// collectively for distinct varieties of Arabic in part 3 of ISO 639.
    ///
    Macro,
    /// Collections of languages
    ///
    /// A collective language code element is an identifier that represents a
    /// group of individual languages that are not deemed to be one language
    /// in any usage context. Whereas ISO 639-2 includes three-letter
    /// identifiers for such collections of languages, this part of ISO 639
    /// provides identifiers for individual languages and macrolanguages only.
    ///
    Collection,
    /// Dialects
    ///
    /// The linguistic varieties denoted by each of the identifiers in this
    /// part of ISO 639 are assumed to be distinct languages and not dialects
    /// of other languages, even though for some purposes some users may
    /// consider a variety listed in this part of ISO 639 to be a "dialect"
    /// rather than a "language". In this standard, the term dialect is used
    /// as in the field of linguistics where it simply identifies any
    /// sub-variety of a language such as might be based on geographic region,
    /// age, gender, social class, time period, or the like. This contrasts
    /// with a popular usage in which "dialect" is typically construed to
    /// connote a substandard or undeveloped form of language.
    ///
    ///The dialects of a language are included within the denotation
    /// represented by the identifier for that language. Thus, each language
    /// identifier represents the complete range of all the spoken or written
    /// varieties of that language, including any standardized form.
    ///
    ///
    Dialect,
    /// Reserved for local use
    ///
    /// Identifiers `qaa` through `qtz` are reserved for local use, to be used
    /// in cases in which there is no suitable existing code in ISO 639. There
    /// are no constraints as to scope of denotation. These identifiers may
    /// only be used locally, and may not be used for interchange of data
    /// without a private agreement.
    ///
    Reserved,
    /// Special code elements
    ///
    /// ISO 639-2 defines three code elements for other special situations.
    /// The identifier `mul` (multiple languages) should be applied when many
    /// languages are used and it is not practical to specify all the
    /// appropriate language codes. The identifier `und` (undetermined) is
    /// provided for those situations in which a language or languages must be
    /// indicated but the language cannot be identified. The identifier `zxx`
    /// (no linguistic content) may be applied in a situation in which a
    /// language identifier is required by system definition, but the item
    /// being described does not actually contain linguistic content.
    ///
    Special,
}

///
/// The five language types defined in ISO 639-3 for individual languages.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LanguageType {
    /// Living languages
    ///
    /// A language is listed as living when there are people still living who
    /// learned it as a first language. This part of ISO 639 also includes
    /// identifiers for languages that are no longer living.
    Living,
    /// Extinct languages
    ///
    /// A language is listed as extinct if it has gone extinct in recent
    /// times. (e.g. in the last few centuries). The criteria for identifying
    /// distinct languages in these case are based on intelligibility (as
    /// defined for individual languages).
    Extinct,
    /// Ancient languages
    ///
    /// A language is listed as ancient if it went extinct in ancient times
    /// (e.g. more than a millennium ago). Identifiers are assigned to ancient
    /// languages which have a distinct literature and are treated distinctly
    /// by the scholarly community. It would be ideal to be able to assign
    /// identifiers to ancient languages on the basis of intelligibility, but
    /// ancient records rarely contain enough information to make this
    /// possible. In order to qualify for inclusion in ISO 639-3, the language
    /// must have an attested literature or be well-documented as a language
    /// known to have been spoken by some particular community at some point
    /// in history; it may not be a reconstructed language inferred from
    /// historical-comparative analysis.
    ///
    Ancient,
    /// Historic languages
    ///
    /// A language is listed as historic when it is considered to be distinct
    /// from any modern languages that are descended from it: for instance,
    /// Old English and Middle English. In these cases, the language did not
    /// become extinct; rather, it changed into a different language over
    /// time. Here, too, the criterion is that the language have a literature
    /// that is treated distinctly by the scholarly community.
    ///
    Historic,
    /// Constructed languages
    ///
    /// This part of ISO 639 also includes identifiers that denote constructed
    /// (or artificial) languages. In order to qualify for inclusion the
    /// language must have a literature and it must be designed for the
    /// purpose of human communication. It must be a complete language, and be
    /// in use for human communication by some community long enough to be
    /// passed to a second generation of users. Specifically excluded are
    /// reconstructed languages and computer programming languages.
    Constructed,
    /// Special and Reserved
    ///
    /// Four codes are set aside in ISO 639-2 and ISO 639-3 for cases where
    /// none of the specific codes are appropriate. These are intended
    /// primarily for applications like databases where an ISO code is
    /// required regardless of whether one exists.
    ///
    /// In addition, 520 codes in the range qaa–qtz are 'reserved for local
    /// use'.
    Special,
}

// ------------------------------------------------------------------------------------------------
//
// The rest of this file is generated by the package build script.
//
// ------------------------------------------------------------------------------------------------

include!(concat!(env!("OUT_DIR"), "/part_3.rs"));
