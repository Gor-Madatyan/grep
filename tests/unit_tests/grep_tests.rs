use aho_corasick::MatchKind;

use super::*;

#[test]
fn recomply() {
    let mut config: Config<_> = ["1234", "5678!", "2142"].into();
    config.match_kind(MatchKind::LeftmostLongest);
    let mut grep = config.build_grep("GI12H_5678!_G".to_owned());
    let new_matches = ["4232", "3424566"];
    grep.config.matches = new_matches.into();
    grep.recomply();
    assert_eq!(grep.pattern_count(), new_matches.len());
    assert_eq!(grep.config.get_matches(), new_matches);
    assert_eq!(grep.match_kind(), &MatchKind::LeftmostLongest);
}


#[test]
fn set_matches() {
    let matches = ["432!!!", "321!", "12d4$"];
    let mut config: Config<_> = matches.into();
    config.match_kind(MatchKind::LeftmostLongest);
    let mut grep = config.build_grep("432!!!, 12d4$ dea3");
    let new_matches = ["RTEWDA", "BNVDSS213"];
    grep.set_matches(new_matches);
    assert_eq!(grep.match_kind(), &MatchKind::LeftmostLongest);
    assert_eq!(grep.config.get_matches(), new_matches);
    assert_eq!(grep.pattern_count(), new_matches.len())
}

#[test]
fn set_matches_and_autoconfigure() {
    let matches = ["432!!!", "321!", "12d4$"];
    let mut config: Config<_> = matches.into();
    config.match_kind(MatchKind::LeftmostLongest);
    config.auto_configure();
    let mut grep = config.build_grep("432!!!321!&&");
    let new_matches = ["12314", "czxc", "12314", "3886", "12314",
        "3886", "12314", "3886", "cccc", "aaa", "12314", "wwdda", "dassad", "38342", "12314", "3886", "12314",
        "3886", "aw2332", "3886", "12314", "3886", "12314", "3886", "12314", "3886"];
    grep.set_matches_and_auto_configure(new_matches);
    assert_eq!(grep.config.get_matches(), new_matches);
    assert_eq!(grep.match_kind(), &MatchKind::LeftmostLongest);
    assert_eq!(grep.pattern_count(), new_matches.len())
}