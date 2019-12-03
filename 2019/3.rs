use std::collections::{HashSet, HashMap};

enum Dir { R, D, U, L }
use Dir::*;

impl Dir {
    fn step_one(&self, pos: (i32, i32)) -> (i32, i32) {
        let (x, y) = pos;
        match self {
            R => (x + 1, y),
            D => (x, y - 1),
            U => (x, y + 1),
            L => (x - 1, y),
        }
    }
}

fn main() {
    let wire1 = [
        (R, 1005), (D, 32), (R, 656), (U, 228), (L, 629), (U, 59), (L, 558), (D, 366), (L, 659), (D, 504), (R, 683), (U, 230), (R, 689), (U, 489), (R, 237), (U, 986), (L, 803), (U, 288), (R, 192), (D, 473), (L, 490), (U, 934), (L, 749), (D, 631), (L, 333), (U, 848), (L, 383), (D, 363), (L, 641), (D, 499), (R, 926), (D, 945), (L, 520), (U, 311), (R, 75), (D, 414), (L, 97), (D, 338), (L, 754), (U, 171), (R, 601), (D, 215), (R, 490), (U, 164), (R, 158), (U, 499), (L, 801), (U, 27), (L, 671), (D, 552), (R, 406), (U, 168), (R, 12), (D, 321), (L, 97), (U, 27), (R, 833), (U, 503), (R, 950), (U, 432), (L, 688), (U, 977), (R, 331), (D, 736), (R, 231), (U, 301), (L, 579), (U, 17), (R, 984), (U, 399), (L, 224), (U, 100), (L, 266), (U, 184), (R, 46), (D, 989), (L, 851), (D, 739), (R, 45), (D, 231), (R, 893), (D, 372), (L, 260), (U, 26), (L, 697), (U, 423), (L, 716), (D, 573), (L, 269), (U, 867), (R, 722), (U, 193), (R, 889), (D, 322), (L, 743), (U, 371), (L, 986), (D, 835), (R, 534), (U, 170), (R, 946), (U, 271), (L, 514), (D, 521), (L, 781), (U, 390), (L, 750), (D, 134), (L, 767), (U, 599), (L, 508), (U, 683), (L, 426), (U, 433), (L, 405), (U, 10), (L, 359), (D, 527), (R, 369), (D, 365), (L, 405), (D, 812), (L, 979), (D, 122), (L, 782), (D, 460), (R, 583), (U, 765), (R, 502), (D, 2), (L, 109), (D, 69), (L, 560), (U, 76), (R, 130), (D, 794), (R, 197), (D, 113), (L, 602), (D, 123), (L, 190), (U, 246), (L, 407), (D, 957), (L, 35), (U, 41), (L, 884), (D, 591), (R, 38), (D, 911), (L, 269), (D, 204), (R, 332), (U, 632), (L, 826), (D, 202), (L, 984), (U, 153), (L, 187), (U, 472), (R, 272), (U, 232), (L, 786), (U, 932), (L, 618), (U, 104), (R, 632), (D, 469), (L, 868), (D, 451), (R, 261), (U, 647), (L, 211), (D, 781), (R, 609), (D, 549), (L, 628), (U, 963), (L, 917), (D, 716), (L, 218), (U, 71), (L, 148), (U, 638), (R, 34), (U, 133), (R, 617), (U, 312), (L, 215), (D, 41), (L, 673), (U, 643), (R, 379), (U, 486), (L, 273), (D, 539), (L, 294), (D, 598), (L, 838), (D, 60), (L, 158), (U, 817), (R, 207), (U, 825), (L, 601), (D, 786), (R, 225), (D, 89), (L, 417), (U, 481), (L, 416), (U, 133), (R, 261), (U, 405), (R, 109), (U, 962), (R, 104), (D, 676), (R, 966), (U, 138), (L, 343), (U, 14), (L, 82), (U, 564), (R, 73), (D, 361), (R, 678), (D, 868), (L, 273), (D, 879), (R, 629), (U, 164), (R, 228), (U, 949), (R, 504), (D, 254), (L, 662), (D, 726), (R, 126), (D, 437), (R, 569), (D, 23), (R, 246), (U, 840), (R, 457), (D, 429), (R, 296), (U, 110), (L, 984), (D, 106), (L, 44), (U, 264), (R, 801), (D, 350), (R, 932), (D, 334), (L, 252), (U, 714), (L, 514), (U, 261), (R, 632), (D, 926), (R, 944), (U, 924), (R, 199), (D, 181), (L, 737), (U, 408), (R, 636), (U, 57), (L, 380), (D, 949), (R, 557), (U, 28), (L, 432), (D, 83), (R, 829), (D, 865), (L, 902), (D, 351), (R, 71), (U, 704), (R, 477), (D, 501), (L, 882), (D, 75), (R, 325), (D, 53), (L, 990), (U, 460), (R, 165), (D, 82), (R, 577), (D, 788), (R, 375), (U, 264), (L, 178), (D, 193), (R, 830), (D, 343), (L, 394)
    ];

    let wire2 = [
        (L, 1003), (U, 125), (L, 229), (U, 421), (R, 863), (D, 640), (L, 239), (U, 580), (R, 342), (U, 341), (R, 989), (U, 732), (R, 51), (U, 140), (L, 179), (U, 60), (R, 483), (D, 575), (R, 49), (U, 220), (L, 284), (U, 336), (L, 905), (U, 540), (L, 392), (U, 581), (L, 570), (U, 446), (L, 817), (U, 694), (R, 923), (U, 779), (R, 624), (D, 387), (R, 495), (D, 124), (R, 862), (D, 173), (R, 425), (D, 301), (L, 550), (D, 605), (R, 963), (U, 503), (R, 571), (U, 953), (L, 878), (D, 198), (L, 256), (D, 77), (R, 409), (D, 752), (R, 921), (D, 196), (R, 977), (U, 86), (L, 842), (U, 155), (R, 987), (D, 39), (L, 224), (U, 433), (L, 829), (D, 99), (R, 558), (U, 736), (R, 645), (D, 335), (L, 52), (D, 998), (L, 613), (D, 239), (R, 470), (U, 79), (R, 839), (D, 71), (L, 753), (U, 127), (R, 135), (D, 429), (R, 729), (U, 71), (L, 151), (U, 875), (R, 668), (D, 220), (L, 501), (D, 822), (R, 306), (D, 557), (R, 461), (U, 942), (R, 59), (U, 14), (R, 353), (D, 546), (R, 409), (D, 261), (R, 204), (U, 873), (L, 847), (U, 936), (R, 611), (U, 487), (R, 474), (U, 406), (R, 818), (U, 838), (L, 301), (D, 684), (R, 861), (D, 738), (L, 265), (D, 214), (R, 272), (D, 702), (L, 145), (U, 872), (R, 345), (D, 623), (R, 200), (D, 186), (R, 407), (U, 988), (L, 608), (U, 533), (L, 185), (D, 287), (L, 549), (U, 498), (L, 630), (U, 295), (L, 425), (U, 517), (L, 263), (D, 27), (R, 697), (U, 177), (L, 615), (U, 960), (L, 553), (U, 974), (L, 856), (U, 716), (R, 126), (D, 819), (L, 329), (D, 233), (L, 212), (U, 232), (L, 164), (D, 712), (R, 316), (D, 682), (L, 641), (U, 676), (L, 535), (U, 783), (R, 39), (U, 953), (R, 39), (U, 511), (R, 837), (U, 325), (R, 391), (U, 401), (L, 642), (U, 435), (R, 626), (U, 801), (R, 876), (D, 849), (R, 448), (D, 8), (R, 74), (U, 238), (L, 186), (D, 558), (L, 648), (D, 258), (R, 262), (U, 7), (L, 510), (U, 178), (L, 183), (U, 415), (L, 631), (D, 162), (L, 521), (D, 910), (R, 462), (U, 789), (R, 885), (D, 822), (R, 908), (D, 879), (R, 614), (D, 119), (L, 570), (U, 831), (R, 993), (U, 603), (L, 118), (U, 764), (L, 414), (U, 39), (R, 14), (U, 189), (L, 415), (D, 744), (R, 897), (U, 714), (R, 326), (U, 348), (R, 822), (U, 98), (L, 357), (D, 478), (L, 464), (D, 851), (L, 545), (D, 241), (L, 672), (U, 197), (R, 156), (D, 916), (L, 246), (U, 578), (R, 4), (U, 195), (R, 82), (D, 402), (R, 327), (D, 429), (R, 119), (U, 661), (L, 184), (D, 122), (R, 891), (D, 499), (L, 808), (U, 519), (L, 36), (U, 323), (L, 259), (U, 479), (L, 647), (D, 354), (R, 891), (D, 320), (R, 653), (U, 772), (L, 158), (U, 608), (R, 149), (U, 564), (L, 164), (D, 998), (L, 485), (U, 107), (L, 145), (U, 834), (R, 846), (D, 462), (L, 391), (D, 661), (R, 841), (U, 742), (L, 597), (D, 937), (L, 92), (U, 877), (L, 350), (D, 130), (R, 684), (U, 914), (R, 400), (D, 910), (L, 739), (U, 789), (L, 188), (U, 256), (R, 10), (U, 258), (L, 965), (U, 942), (R, 234), (D, 106), (R, 852), (U, 108), (R, 732), (U, 339), (L, 955), (U, 271), (L, 340), (U, 23), (R, 373), (D, 100), (R, 137), (U, 648), (L, 130)
    ];

    let crossed1 = crossed_tiles(&wire1);
    let crossed2 = crossed_tiles(&wire2);

    let crossed1_positions: HashSet<_> = crossed1.keys().collect();
    let crossed2_positions: HashSet<_> = crossed2.keys().collect();

    let intersect: Vec<_> = crossed1_positions.intersection(&crossed2_positions).collect();

    let mut min_dist = std::i32::MAX;
    let mut min_cost = std::i32::MAX;
    for pos in &intersect {
        let (x, y) = pos;
        let dist = x.abs() + y.abs();
        if dist < min_dist {
            min_dist = dist;
        }

        let cost = crossed1[pos] + crossed2[pos];
        if cost < min_cost {
            min_cost = cost;
        }
    }
    println!("part 1: {}", min_dist);
    println!("part 2: {}", min_cost);
}

fn crossed_tiles(segments: &[(Dir, i32)]) -> HashMap<(i32, i32), i32> {
    let mut crossed = HashMap::new();
    let mut pos = (0, 0);
    let mut dist = 0;

    for (dir, len) in segments {
        for _ in 0..*len {
            dist += 1;
            pos = dir.step_one(pos);
            crossed.insert(pos, dist);
        }
    }

    crossed
}
