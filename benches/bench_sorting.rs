use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_sort::LexicalSort;

// 100 auto-generated strings with a length between 5 and 20 characters
//
// Half of them have another string with a common prefix:
//  - 50 strings whose first character is unique
//  - 12 pairs with a common prefix of length 1
//  - 6 pairs with a common prefix of length 2
//  - 2 pairs with a common prefix of length 3
//  - 2 pairs with a common prefix of length 4
//  - 2 pairs with a common prefix of length 6
//  - 1 pair with a common prefix of length 10 ("pŊcóh~öJL*fæL", "pŊcóh~öJL*Ł")
//
// The strings are shuffled randomly.
#[rustfmt::skip]
static STRINGS: [&str; 100] = [
    "dT@ŁeΩØä.µ#", "ŋ +GÐNSCEṇZæB+łßc", "Øí mÞt@Bwi", "+9Üyđg", "ZAgL!Gé",
    "NE€j̣đm¼ŁCBLWHjŊ", "zvħü¼Ø!iµ¼.", "ŁøŒSBN9", "(̣¥AŒvŁy40", "æ#áBV(AŁFAdQ. 8",
    "#½IṣÞY²̣", "9f1kẞIz7æ3€ßeŁ", "ŧywΩEc", "!½j.œ³d⅞qÖDVÜðQNŒ", "Äþ+RF ",
    "¼µírT²ałDKnwB.T8P", "€ŋØáEIJt @éµ", "³3ömrn@H2jóĸ#R", "Ŋđ⅞y$VṚ)ĦE",
    "ŧF€zÐxLúT7²ẒXŧ̣", "ÜDµ̣LNĸd²Öh€Æn~̣", "ä1v9⅞$ĸ⅝Äh.91Þ.#", "Ló1vó ḄZkḍíT€",
    "½5uØeámΩ", "ZAþmD²VfHṂe", "1cŒCu8f½¼4ÞáeŒΩq", "F+úÖsI½f#n7GfV#Æ¼",
    "TÆạÜGbUjſfDnŁ", "Uþĸ 7Ä$öáx⅝ävyU", " ÄÐEØ*", "Ω9äülq6H", "Ω9äülq.o",
    "E1Ö0AD5̣SM(̣", "QvlkRiTŋúÐa", " ̣yZ$!ÄZ¥", "$1pđÞúþĦ¼GSWħØW", "úð5 +ẞŊÞi1ðTðŋ",
    "!½j.BΩ*", "tpHV$eḳ", "7 ⅝ Eiæ#̣aŊum³3ŊR", "iJ̣P)F", "½plŒügäGŊFħPł3¹Ö8Ω",
    "BĦ$Œö8¹̣úH", "Ħæä3v³nzÖc", "Iøþænóœ", "qi⅝qzH ⅝", "Rg@W6Æ0ịI9¹h)", "fÖ¼Æ¼üÜ²",
    "3ΩŒŁá²hŊOZ½AŁndt", "F1Ðøéu½áR", "̣lzłäÞ", "yſ8hẞ¹²zọ́ry", "ßímJĸ+", "Ðr1LwO²øvØ",
    "Ä2a0DÜvMØΩ)íÆ9Ŋ̣", "8þJ⅝UÞΩ", "Æ²ḲŧĦFGÐŊä$s83h", "DGbŁbœfP", "kHö9̣xps b!yOlV#",
    "~+yĦ$³LI", "nTVT²Þµ!Œ", "Ppyg̣Ð.LlYĸẞħ", "µ⅝4CŁQBQp", "ü!HßÞéGzøED@€€",
    "W⅞i.C3Ṛþ", "fÖØ+d1ä", "euA)WE", "DGbŁÖé8!j!DBł9a", "wsRCQßA", "ÐrẠMjdtaj*",
    "ł+¥UŊiÆiÜé3̣Bím", "PpÖụXéẞiLµNHLCÄ", "ég³$ØOxqfbUḲðy L", "pŊcóh~öJL*fæL",
    "8O#SOÞ̣b+⅞ĸ!W!⅞pc", "1cpb6wſPßS2ŒĦ", "Þvð¼ÖbvWf!Oüđ*Zħ5", "JeníéCp*Ö",
    "pŊcóh~öJL*Ł", "i!5ZQ", "E1ÖRŊ€dœ+.Þ", "Hŧ7$SáĦt8B", "yt½Xđm", "ŒE¼p̣N8²",
    "⅞eþ NFáŒ̣+JÄLrQs", "OøqKar~ü", "ÖĸΩÖ ", "hpoXt", "7$eŁPħØ2#AßµµđÐí+Y",
    "đØ@þ³v*¹Bco8µŊhw", "wsRCQß̣*Cg", "#ZvIΩłKẞØ7s~~1x", "²øeXbl3PrŋŁBE1m", "ọOhŁó",
    "nT #2€9ł l", "2XöáUbúeV", "µ⅝40uVøQUD+̣", "YdBycð@ọ́D!¼cp8", "dünÆéÆ@µZ!f+",
    "OJoÄXPŊ",
];

// 100 strings containing the prefix "T-", followed by a number
// of random length between 1 and 8 digits.
#[rustfmt::skip]
static NUM_STRINGS: [&str; 100] = [
    "T-60575", "T-49", "T-91237", "T-31512", "T-12043", "T-3012138", "T-2008720",
    "T-160", "T-111246", "T-19104", "T-1948241", "T-10130396", "T-1103", "T-5067",
    "T-53403082", "T-66774788", "T-15", "T-162166", "T-3774328", "T-5", "T-248913",
    "T-202", "T-521749", "T-125", "T-7631192", "T-38", "T-318", "T-639390", "T-157",
    "T-61", "T-3798339", "T-27802", "T-14384042", "T-12229308", "T-2", "T-3903",
    "T-89476", "T-112", "T-744113", "T-236", "T-20137", "T-23527", "T-4810796",
    "T-11", "T-354", "T-8594", "T-2", "T-12076", "T-1206524", "T-3554909", "T-1",
    "T-12", "T-49366", "T-16249281", "T-51", "T-472", "T-1586", "T-16112257", "T-12",
    "T-139", "T-14624", "T-110", "T-16603", "T-5254907", "T-61", "T-8055", "T-44",
    "T-11", "T-111", "T-369", "T-64009992", "T-7", "T-515833", "T-3", "T-51",
    "T-122075", "T-13", "T-1669", "T-561456", "T-252344", "T-9371073", "T-361096",
    "T-5252431", "T-33587", "T-2", "T-5098095", "T-182", "T-3568008", "T-11559",
    "T-13", "T-653", "T-1085", "T-13249", "T-932722", "T-3036859", "T-52065", "T-23189",
    "T-114", "T-40716", "T-145243",
];

pub fn sort_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("Random strings");
    group.bench_function("sort 100 strings lexically + naturaly", |b| {
        b.iter_with_large_setup(
            || black_box(STRINGS.clone()),
            |strings: [&str; 100]| {
                black_box(strings).lexical_sort(true);
                strings
            },
        );
    });
    group.bench_function("sort 100 strings lexically", |b| {
        b.iter_with_large_setup(
            || black_box(STRINGS.clone()),
            |strings: [&str; 100]| {
                black_box(strings).lexical_sort(false);
                strings
            },
        );
    });
    group.bench_function("sort 100 strings alphanumerically", |b| {
        b.iter_with_large_setup(
            || black_box(STRINGS.clone()),
            |strings: [&str; 100]| {
                alphanumeric_sort::sort_str_slice(&mut black_box(strings));
                strings
            },
        );
    });
    group.bench_function("sort 100 strings natively", |b| {
        b.iter_with_large_setup(
            || black_box(STRINGS.clone()),
            |strings: [&str; 100]| {
                black_box(strings).sort();
                strings
            },
        );
    });
    group.finish();
}

pub fn sort_numbers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Strings with numbers");
    group.bench_function("sort 100 numeric strings lexically + naturaly", |b| {
        b.iter_with_large_setup(
            || black_box(NUM_STRINGS.clone()),
            |strings: [&str; 100]| {
                black_box(strings).lexical_sort(true);
                strings
            },
        );
    });
    group.bench_function("sort 100 numeric strings lexically", |b| {
        b.iter_with_large_setup(
            || black_box(NUM_STRINGS.clone()),
            |strings: [&str; 100]| {
                black_box(strings).lexical_sort(false);
                strings
            },
        );
    });
    group.bench_function("sort 100 numeric strings alphanumerically", |b| {
        b.iter_with_large_setup(
            || black_box(NUM_STRINGS.clone()),
            |strings: [&str; 100]| {
                alphanumeric_sort::sort_str_slice(&mut black_box(strings));
                strings
            },
        );
    });
    group.bench_function("sort 100 numeric strings natively", |b| {
        b.iter_with_large_setup(
            || black_box(NUM_STRINGS.clone()),
            |strings: [&str; 100]| {
                black_box(strings).sort();
                strings
            },
        );
    });
    group.finish();
}

criterion_group!(sorting, sort_strings, sort_numbers);
criterion_main!(sorting);
