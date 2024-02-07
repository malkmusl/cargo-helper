(function() {var implementors = {
"cargo_helper":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"enum\" href=\"cargo_helper/enum.LICENSE.html\" title=\"enum cargo_helper::LICENSE\">LICENSE</a>",1,["cargo_helper::LICENSE"]]],
"chrono":[["impl Freeze for <a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>",1,["chrono::duration::Duration"]],["impl&lt;Tz&gt; Freeze for <a class=\"struct\" href=\"chrono/struct.Date.html\" title=\"struct chrono::Date\">Date</a>&lt;Tz&gt;<div class=\"where\">where\n    &lt;Tz as <a class=\"trait\" href=\"chrono/trait.TimeZone.html\" title=\"trait chrono::TimeZone\">TimeZone</a>&gt;::<a class=\"associatedtype\" href=\"chrono/trait.TimeZone.html#associatedtype.Offset\" title=\"type chrono::TimeZone::Offset\">Offset</a>: Freeze,</div>",1,["chrono::date::Date"]],["impl&lt;Tz&gt; Freeze for <a class=\"struct\" href=\"chrono/struct.DateTime.html\" title=\"struct chrono::DateTime\">DateTime</a>&lt;Tz&gt;<div class=\"where\">where\n    &lt;Tz as <a class=\"trait\" href=\"chrono/trait.TimeZone.html\" title=\"trait chrono::TimeZone\">TimeZone</a>&gt;::<a class=\"associatedtype\" href=\"chrono/trait.TimeZone.html#associatedtype.Offset\" title=\"type chrono::TimeZone::Offset\">Offset</a>: Freeze,</div>",1,["chrono::datetime::DateTime"]],["impl&lt;I&gt; Freeze for <a class=\"struct\" href=\"chrono/format/struct.DelayedFormat.html\" title=\"struct chrono::format::DelayedFormat\">DelayedFormat</a>&lt;I&gt;<div class=\"where\">where\n    I: Freeze,</div>",1,["chrono::format::formatting::DelayedFormat"]],["impl Freeze for <a class=\"enum\" href=\"chrono/format/enum.SecondsFormat.html\" title=\"enum chrono::format::SecondsFormat\">SecondsFormat</a>",1,["chrono::format::formatting::SecondsFormat"]],["impl Freeze for <a class=\"struct\" href=\"chrono/format/struct.Parsed.html\" title=\"struct chrono::format::Parsed\">Parsed</a>",1,["chrono::format::parsed::Parsed"]],["impl&lt;'a&gt; Freeze for <a class=\"struct\" href=\"chrono/format/strftime/struct.StrftimeItems.html\" title=\"struct chrono::format::strftime::StrftimeItems\">StrftimeItems</a>&lt;'a&gt;",1,["chrono::format::strftime::StrftimeItems"]],["impl Freeze for <a class=\"enum\" href=\"chrono/format/enum.Pad.html\" title=\"enum chrono::format::Pad\">Pad</a>",1,["chrono::format::Pad"]],["impl Freeze for <a class=\"enum\" href=\"chrono/format/enum.Numeric.html\" title=\"enum chrono::format::Numeric\">Numeric</a>",1,["chrono::format::Numeric"]],["impl Freeze for <a class=\"struct\" href=\"chrono/format/struct.InternalNumeric.html\" title=\"struct chrono::format::InternalNumeric\">InternalNumeric</a>",1,["chrono::format::InternalNumeric"]],["impl Freeze for <a class=\"enum\" href=\"chrono/format/enum.Fixed.html\" title=\"enum chrono::format::Fixed\">Fixed</a>",1,["chrono::format::Fixed"]],["impl Freeze for <a class=\"struct\" href=\"chrono/format/struct.InternalFixed.html\" title=\"struct chrono::format::InternalFixed\">InternalFixed</a>",1,["chrono::format::InternalFixed"]],["impl Freeze for <a class=\"struct\" href=\"chrono/format/struct.OffsetFormat.html\" title=\"struct chrono::format::OffsetFormat\">OffsetFormat</a>",1,["chrono::format::OffsetFormat"]],["impl Freeze for <a class=\"enum\" href=\"chrono/format/enum.OffsetPrecision.html\" title=\"enum chrono::format::OffsetPrecision\">OffsetPrecision</a>",1,["chrono::format::OffsetPrecision"]],["impl Freeze for <a class=\"enum\" href=\"chrono/format/enum.Colons.html\" title=\"enum chrono::format::Colons\">Colons</a>",1,["chrono::format::Colons"]],["impl&lt;'a&gt; Freeze for <a class=\"enum\" href=\"chrono/format/enum.Item.html\" title=\"enum chrono::format::Item\">Item</a>&lt;'a&gt;",1,["chrono::format::Item"]],["impl Freeze for <a class=\"struct\" href=\"chrono/format/struct.ParseError.html\" title=\"struct chrono::format::ParseError\">ParseError</a>",1,["chrono::format::ParseError"]],["impl Freeze for <a class=\"enum\" href=\"chrono/format/enum.ParseErrorKind.html\" title=\"enum chrono::format::ParseErrorKind\">ParseErrorKind</a>",1,["chrono::format::ParseErrorKind"]],["impl Freeze for <a class=\"struct\" href=\"chrono/naive/struct.NaiveWeek.html\" title=\"struct chrono::naive::NaiveWeek\">NaiveWeek</a>",1,["chrono::naive::date::NaiveWeek"]],["impl Freeze for <a class=\"struct\" href=\"chrono/naive/struct.Days.html\" title=\"struct chrono::naive::Days\">Days</a>",1,["chrono::naive::date::Days"]],["impl Freeze for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDate.html\" title=\"struct chrono::naive::NaiveDate\">NaiveDate</a>",1,["chrono::naive::date::NaiveDate"]],["impl Freeze for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDateDaysIterator.html\" title=\"struct chrono::naive::NaiveDateDaysIterator\">NaiveDateDaysIterator</a>",1,["chrono::naive::date::NaiveDateDaysIterator"]],["impl Freeze for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDateWeeksIterator.html\" title=\"struct chrono::naive::NaiveDateWeeksIterator\">NaiveDateWeeksIterator</a>",1,["chrono::naive::date::NaiveDateWeeksIterator"]],["impl Freeze for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDateTime.html\" title=\"struct chrono::naive::NaiveDateTime\">NaiveDateTime</a>",1,["chrono::naive::datetime::NaiveDateTime"]],["impl Freeze for <a class=\"struct\" href=\"chrono/naive/struct.IsoWeek.html\" title=\"struct chrono::naive::IsoWeek\">IsoWeek</a>",1,["chrono::naive::isoweek::IsoWeek"]],["impl Freeze for <a class=\"struct\" href=\"chrono/naive/struct.NaiveTime.html\" title=\"struct chrono::naive::NaiveTime\">NaiveTime</a>",1,["chrono::naive::time::NaiveTime"]],["impl Freeze for <a class=\"struct\" href=\"chrono/offset/struct.FixedOffset.html\" title=\"struct chrono::offset::FixedOffset\">FixedOffset</a>",1,["chrono::offset::fixed::FixedOffset"]],["impl Freeze for <a class=\"struct\" href=\"chrono/offset/struct.Local.html\" title=\"struct chrono::offset::Local\">Local</a>",1,["chrono::offset::local::Local"]],["impl Freeze for <a class=\"struct\" href=\"chrono/offset/struct.Utc.html\" title=\"struct chrono::offset::Utc\">Utc</a>",1,["chrono::offset::utc::Utc"]],["impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"chrono/offset/enum.LocalResult.html\" title=\"enum chrono::offset::LocalResult\">LocalResult</a>&lt;T&gt;<div class=\"where\">where\n    T: Freeze,</div>",1,["chrono::offset::LocalResult"]],["impl Freeze for <a class=\"enum\" href=\"chrono/round/enum.RoundingError.html\" title=\"enum chrono::round::RoundingError\">RoundingError</a>",1,["chrono::round::RoundingError"]],["impl Freeze for <a class=\"enum\" href=\"chrono/enum.Weekday.html\" title=\"enum chrono::Weekday\">Weekday</a>",1,["chrono::weekday::Weekday"]],["impl Freeze for <a class=\"enum\" href=\"chrono/enum.Month.html\" title=\"enum chrono::Month\">Month</a>",1,["chrono::month::Month"]],["impl Freeze for <a class=\"struct\" href=\"chrono/struct.Months.html\" title=\"struct chrono::Months\">Months</a>",1,["chrono::month::Months"]],["impl Freeze for <a class=\"struct\" href=\"chrono/struct.OutOfRange.html\" title=\"struct chrono::OutOfRange\">OutOfRange</a>",1,["chrono::OutOfRange"]]],
"iana_time_zone":[["impl Freeze for <a class=\"enum\" href=\"iana_time_zone/enum.GetTimezoneError.html\" title=\"enum iana_time_zone::GetTimezoneError\">GetTimezoneError</a>",1,["iana_time_zone::GetTimezoneError"]]],
"num_traits":[["impl Freeze for <a class=\"enum\" href=\"num_traits/enum.FloatErrorKind.html\" title=\"enum num_traits::FloatErrorKind\">FloatErrorKind</a>",1,["num_traits::FloatErrorKind"]],["impl Freeze for <a class=\"struct\" href=\"num_traits/struct.ParseFloatError.html\" title=\"struct num_traits::ParseFloatError\">ParseFloatError</a>",1,["num_traits::ParseFloatError"]]],
"termcolor":[["impl Freeze for <a class=\"enum\" href=\"termcolor/enum.ColorChoice.html\" title=\"enum termcolor::ColorChoice\">ColorChoice</a>",1,["termcolor::ColorChoice"]],["impl Freeze for <a class=\"struct\" href=\"termcolor/struct.ColorChoiceParseError.html\" title=\"struct termcolor::ColorChoiceParseError\">ColorChoiceParseError</a>",1,["termcolor::ColorChoiceParseError"]],["impl Freeze for <a class=\"struct\" href=\"termcolor/struct.StandardStream.html\" title=\"struct termcolor::StandardStream\">StandardStream</a>",1,["termcolor::StandardStream"]],["impl&lt;'a&gt; Freeze for <a class=\"struct\" href=\"termcolor/struct.StandardStreamLock.html\" title=\"struct termcolor::StandardStreamLock\">StandardStreamLock</a>&lt;'a&gt;",1,["termcolor::StandardStreamLock"]],["impl Freeze for <a class=\"struct\" href=\"termcolor/struct.BufferedStandardStream.html\" title=\"struct termcolor::BufferedStandardStream\">BufferedStandardStream</a>",1,["termcolor::BufferedStandardStream"]],["impl !Freeze for <a class=\"struct\" href=\"termcolor/struct.BufferWriter.html\" title=\"struct termcolor::BufferWriter\">BufferWriter</a>",1,["termcolor::BufferWriter"]],["impl Freeze for <a class=\"struct\" href=\"termcolor/struct.Buffer.html\" title=\"struct termcolor::Buffer\">Buffer</a>",1,["termcolor::Buffer"]],["impl&lt;W&gt; Freeze for <a class=\"struct\" href=\"termcolor/struct.NoColor.html\" title=\"struct termcolor::NoColor\">NoColor</a>&lt;W&gt;<div class=\"where\">where\n    W: Freeze,</div>",1,["termcolor::NoColor"]],["impl&lt;W&gt; Freeze for <a class=\"struct\" href=\"termcolor/struct.Ansi.html\" title=\"struct termcolor::Ansi\">Ansi</a>&lt;W&gt;<div class=\"where\">where\n    W: Freeze,</div>",1,["termcolor::Ansi"]],["impl Freeze for <a class=\"struct\" href=\"termcolor/struct.ColorSpec.html\" title=\"struct termcolor::ColorSpec\">ColorSpec</a>",1,["termcolor::ColorSpec"]],["impl Freeze for <a class=\"enum\" href=\"termcolor/enum.Color.html\" title=\"enum termcolor::Color\">Color</a>",1,["termcolor::Color"]],["impl Freeze for <a class=\"struct\" href=\"termcolor/struct.ParseColorError.html\" title=\"struct termcolor::ParseColorError\">ParseColorError</a>",1,["termcolor::ParseColorError"]],["impl&lt;'a&gt; Freeze for <a class=\"struct\" href=\"termcolor/struct.HyperlinkSpec.html\" title=\"struct termcolor::HyperlinkSpec\">HyperlinkSpec</a>&lt;'a&gt;",1,["termcolor::HyperlinkSpec"]]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()