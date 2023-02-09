use charname::get_name;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const TEST_CHARS: [u32; 1000] = [
    0x1F32D, 0x1D15C, 0x011C5, 0x1F868, 0x1D1D6, 0xE018D, 0x0A27A, 0x11729, 0x02C16, 0x0146F,
    0x1FBF6, 0x109E4, 0x131DC, 0x02817, 0x0067A, 0x01B6C, 0x1F57C, 0x031E3, 0x0A644, 0x12292,
    0x10F03, 0x11D7F, 0x119B0, 0x00255, 0x11FDB, 0x01039, 0x1B19C, 0x0081E, 0x2F8B6, 0x1D212,
    0x145D2, 0x0A653, 0x02CA1, 0x144AD, 0x10119, 0x1F7C3, 0x031A2, 0x029D0, 0x145ED, 0x01C51,
    0x02671, 0x028C9, 0x122A0, 0x02D35, 0x0AB50, 0x021F9, 0x1CF63, 0x02F84, 0x01DC4, 0x0076A,
    0x0061E, 0x12320, 0x02784, 0x2F82D, 0x2F8CD, 0x106EC, 0x00D3B, 0x01648, 0x1D36E, 0x1D5A2,
    0x028FC, 0x18897, 0x1DA53, 0x1F39B, 0x18C39, 0x02A2F, 0x01BDF, 0x00833, 0x0FA4F, 0x0A441,
    0x1B253, 0x0A7C0, 0x1099B, 0x032BA, 0x0126F, 0x02542, 0x00E49, 0x0A61F, 0x0094A, 0x0088A,
    0x0FB28, 0x1134C, 0x1EE57, 0x0147E, 0x1D12A, 0x0FA30, 0x0014C, 0x12085, 0x1688A, 0x1F7C7,
    0x11032, 0x1D33B, 0x14470, 0x11A5E, 0x1E2ED, 0x013E5, 0x1D819, 0x0049B, 0x1341A, 0x01928,
    0x12FEF, 0x1E839, 0x018BF, 0x1200D, 0x1054C, 0x1061C, 0x130D5, 0x01085, 0x01532, 0x13110,
    0x00C7D, 0x1D47A, 0x1D58F, 0x13392, 0x00B4D, 0x10432, 0x0FCB8, 0x1E93F, 0x12423, 0x0FE9E,
    0x1D689, 0x10527, 0x00638, 0x188A2, 0x0F9E6, 0x1CF85, 0x01FEB, 0x0083A, 0x2F9A1, 0x00FA5,
    0x02220, 0x00FD6, 0x1FAD6, 0x1F014, 0x2F913, 0x0021B, 0x11339, 0x019E0, 0x0314B, 0x00B21,
    0x1F4B1, 0x02841, 0x13232, 0x0FC06, 0x1D0CA, 0x1F1EC, 0x16A08, 0x0A272, 0x0F9E4, 0x0A360,
    0x130A1, 0x00715, 0x00936, 0x1D0B9, 0x1F126, 0x1E84A, 0x11A71, 0x1F3BC, 0x1F485, 0x01C00,
    0x016B3, 0x0FA8F, 0x132D5, 0x01500, 0x1F6AA, 0x0A66D, 0x16856, 0x002B2, 0x18A9C, 0x01B1A,
    0x02099, 0x1451E, 0x1233A, 0x10CDB, 0x0A883, 0x1F190, 0x01D31, 0x0D7DD, 0x018C4, 0x02B43,
    0x1881A, 0x0A17D, 0x00A42, 0x02C42, 0xE004F, 0x1BC6A, 0x1303C, 0x10751, 0x16804, 0x16E85,
    0x1B264, 0x1F0F0, 0x0A827, 0x02CB5, 0x00018, 0x120D3, 0x00711, 0x02698, 0x1D52F, 0x10453,
    0x001D4, 0x11903, 0x1110E, 0x16F9B, 0x0260B, 0x0172C, 0x011DC, 0x01359, 0x1DA4D, 0x01093,
    0x18853, 0x1062B, 0x023F9, 0x18841, 0x1E883, 0x00BAF, 0x02DA5, 0x169A0, 0x0FA74, 0x10716,
    0x1118B, 0x1687F, 0x02C72, 0x01427, 0x02678, 0x006C8, 0x1DA68, 0x02F48, 0x0FAD0, 0x10531,
    0x168C6, 0x1CFB5, 0x120C0, 0x006B2, 0x1F6FB, 0x00C23, 0x01D5E, 0x1229F, 0x1F5EF, 0x0071F,
    0x0AB32, 0x0FC13, 0x0A8C4, 0x01585, 0x13267, 0x16962, 0x1121A, 0x021D5, 0x01C45, 0x1B0CE,
    0x1FA4C, 0x18C5F, 0x14598, 0x029EC, 0x02E53, 0x106C7, 0x1456E, 0x18C7E, 0x01B1F, 0x10098,
    0x01246, 0x0D7C0, 0x10C13, 0x00568, 0x0162A, 0x1F7AD, 0x0F92C, 0x0A6C1, 0x03261, 0x1D9C9,
    0x0FFCB, 0x021F1, 0x00AB2, 0x0A005, 0x03297, 0x1D69B, 0x169D3, 0x1FB5A, 0x11291, 0x18C3F,
    0x0FB24, 0x0A94B, 0x1D720, 0x01412, 0x031AB, 0x1D34E, 0x0120A, 0x18B5D, 0x118C4, 0x0A533,
    0x0FCF4, 0xE0107, 0x0069D, 0x03291, 0x1038C, 0x1F4C2, 0x01EF4, 0x02E41, 0x1F342, 0x1F1F5,
    0x1ECA3, 0x017BD, 0x100E9, 0x0FE5B, 0x2F97C, 0x008CD, 0x116B4, 0x0060D, 0x005E4, 0x16993,
    0x01D3D, 0x1D594, 0x02D55, 0x10516, 0x0328C, 0x2F8A4, 0x2F809, 0x1D176, 0x1F391, 0x16F01,
    0x122D8, 0x16A70, 0x2F908, 0x14580, 0x0A4E3, 0x030A9, 0x02075, 0x1100A, 0x01310, 0x01589,
    0x0A1E7, 0x1F992, 0x0FB64, 0x18B5F, 0x1F020, 0x00FCE, 0x02386, 0x11819, 0x0F9D2, 0x1181B,
    0x02027, 0x12226, 0x102B7, 0x10841, 0x02CBD, 0x0AAEF, 0x01CA7, 0x1FBB5, 0x1078A, 0x1F698,
    0x1163E, 0x1BC9C, 0x001BB, 0x0FCDC, 0x1D955, 0x01C5C, 0x1D193, 0x1F05F, 0x2F971, 0x11D54,
    0x1B22B, 0x00559, 0x0A57D, 0x0FD2F, 0x0FE0C, 0x1FBBE, 0x02296, 0x0ABA6, 0x0281D, 0x028C0,
    0x025A5, 0x015DD, 0x18BB3, 0x1B003, 0x0A116, 0x1D940, 0x01865, 0x0A92A, 0x118E7, 0x1F06E,
    0x1FA07, 0x0FB29, 0x16A0D, 0x02B7D, 0x1332D, 0x0F9E8, 0x0322A, 0x02D42, 0x00D8D, 0x12267,
    0x1D50D, 0x1D520, 0x02CB2, 0x189D6, 0x11C3D, 0x0338F, 0x1F683, 0x10CCB, 0x0A6EE, 0x02201,
    0x1684C, 0x014C8, 0x02DFA, 0x00659, 0x0A0F9, 0x01F2F, 0x12533, 0x1201A, 0x02461, 0x012C8,
    0x00195, 0x18BA0, 0x1E871, 0x02B48, 0x0A938, 0x01D8F, 0x00DB8, 0x006F7, 0x16B89, 0x02586,
    0x0324A, 0x10B23, 0x014DA, 0x016CA, 0x0FF3F, 0x144C1, 0x0A306, 0x13424, 0x0FA27, 0x2F81E,
    0x02E57, 0x106F9, 0x10923, 0x1E89D, 0x1BC1E, 0x00F52, 0x119C1, 0x1F51C, 0x0FED3, 0x021C5,
    0x0189F, 0x0A1F1, 0x0300D, 0x10B1F, 0x1E882, 0x02910, 0x11022, 0x02A95, 0x03229, 0x18C64,
    0x0287F, 0x1F5B8, 0x10A42, 0x0246B, 0x12361, 0xE0045, 0x0A41F, 0x1D89B, 0x11F54, 0x0A2DC,
    0x02A05, 0x1FA06, 0x2F948, 0x0A309, 0x18C89, 0x0A1B6, 0x019D2, 0x019B8, 0x0FB56, 0x1E2E8,
    0x0A3AF, 0x02258, 0x18CAB, 0x00AEB, 0x1145B, 0x0A980, 0x01B02, 0x031D3, 0x1D9BD, 0x1306A,
    0x10D13, 0x0A033, 0x01562, 0x0A7BD, 0x0A331, 0x0228F, 0x120F5, 0x00A91, 0x00606, 0x1244B,
    0x013D5, 0x100DB, 0x1F008, 0x01F3A, 0x1D824, 0x1E2A6, 0x1D4E3, 0x10B20, 0x01ACE, 0x00129,
    0x00059, 0x16A8E, 0x1CF64, 0x120EE, 0x1D5EB, 0x124A4, 0x0A4C3, 0x0A721, 0x0A5C6, 0x02B5D,
    0x18B11, 0x2FA0A, 0x01384, 0x1226D, 0x024EB, 0x00979, 0x00809, 0x01F45, 0x1193B, 0x017BE,
    0x11ACF, 0x1ECA2, 0x00B72, 0x0033C, 0x04DC0, 0x16F77, 0x100C1, 0x009A4, 0x0117F, 0x1892E,
    0x1EEB5, 0x013CD, 0x1DA20, 0x11D7D, 0x1D573, 0x1B254, 0x00AAD, 0x033F7, 0x01DFC, 0x006BC,
    0x02112, 0x0A698, 0x00254, 0x017A7, 0x1228B, 0x0A6CE, 0x13195, 0x01B25, 0x00154, 0x01C58,
    0x0070F, 0x1E006, 0x1D580, 0x1B0FF, 0x02B52, 0x116B6, 0x107A4, 0x10B24, 0x00490, 0x0153B,
    0x0FB5B, 0x01458, 0x131C3, 0x0FC54, 0x1E4D5, 0x0FBB9, 0x1F9B8, 0x0FFFC, 0x2F888, 0x1E12C,
    0x144D6, 0x01FA9, 0x12FE0, 0x0FC4D, 0x144EF, 0x145A7, 0x1EE02, 0x028A5, 0x16B33, 0x0253A,
    0x1D463, 0x0A81E, 0x0A4DD, 0x10671, 0x03157, 0x1315A, 0x1EC82, 0x1D64E, 0x0A170, 0x1F4DD,
    0x189A4, 0x2F96E, 0x02719, 0x11A02, 0x12484, 0x1D10D, 0x01A60, 0x01E35, 0x02C20, 0x168E2,
    0x10355, 0x02E3C, 0x18871, 0x02F5D, 0x0028D, 0x01201, 0x01930, 0x020ED, 0x02278, 0x14644,
    0x0117B, 0x0276B, 0x10787, 0x0129F, 0x0113A, 0x026FF, 0x021A4, 0x1D135, 0x00101, 0x0FA99,
    0x10C3F, 0x0FB02, 0x18B8C, 0x120AB, 0x130D4, 0x1D5B3, 0x013CC, 0x02AD6, 0x0A231, 0x01472,
    0x10A27, 0x11FC1, 0x1F7D6, 0x1173D, 0x0FC7E, 0x0A09B, 0x0FE3E, 0x02772, 0x1D671, 0x004F3,
    0x00203, 0x10B7B, 0x1F53C, 0x10890, 0x02F68, 0x0A05D, 0x12233, 0x0FF62, 0x02BE6, 0x0160E,
    0x124E8, 0x017D8, 0x10B15, 0x0288C, 0x0A2B6, 0x11CA1, 0x11912, 0x1D78B, 0x10B10, 0x1D5D3,
    0x100DF, 0x0A5DC, 0x0A6C3, 0x11C18, 0x0162E, 0x010F5, 0x019A3, 0x013ED, 0x120D7, 0x0A1DA,
    0x2FA02, 0x13263, 0x1B096, 0x009BF, 0x01C66, 0x02877, 0x12497, 0x0FFBA, 0x1E124, 0x00015,
    0x18908, 0x1DF02, 0x132ED, 0x131F8, 0x1D4E2, 0x131B3, 0x10368, 0x019E4, 0x2F919, 0x0FD7A,
    0x01CB0, 0x1BC2C, 0x1F706, 0x01209, 0x12310, 0x10B5A, 0x0A683, 0x019F3, 0x11210, 0x1F4FD,
    0x1F527, 0x0A5FA, 0x01FE5, 0x01DE5, 0x02B26, 0x2F9BE, 0x16E46, 0x01E28, 0x02131, 0x00B6D,
    0x144D8, 0x11167, 0x1F97E, 0x10621, 0x14472, 0x1B17B, 0x0FA23, 0x01870, 0x01150, 0x11945,
    0x1221A, 0x01C07, 0x16940, 0x130D9, 0x16877, 0x1F92A, 0x01DAA, 0x1F693, 0x002EE, 0x1F071,
    0x10A19, 0x1F3D5, 0x2F87D, 0x0A8EF, 0x1D979, 0x133E6, 0x16FE2, 0x1F13D, 0x108E1, 0x1F682,
    0x1E2DB, 0x189C5, 0x1EE6C, 0x1D42D, 0x1EE88, 0x0251D, 0x03356, 0x111DB, 0x1D9E4, 0x1F128,
    0x0A1D7, 0x12043, 0x132AE, 0x0FA2D, 0x02FF9, 0x10473, 0x116B5, 0x1FA2E, 0x01F6D, 0x1B2C9,
    0x00F8A, 0x026CE, 0x106D8, 0x1F9DF, 0x11597, 0x02779, 0x02A70, 0x0A4DA, 0x130F9, 0x01EEC,
    0x1D17E, 0x114B1, 0x1D6D2, 0x1D79A, 0x1072E, 0x1F407, 0x01604, 0x118A2, 0x1D080, 0x0AB91,
    0x1D416, 0x0A725, 0x16B41, 0x0324F, 0x1D701, 0x1F597, 0x0120B, 0x031CC, 0xE006D, 0x121EE,
    0x00797, 0x1B1E7, 0x11691, 0x03251, 0x01286, 0x2F94E, 0x11623, 0x1F665, 0x110D7, 0x021B1,
    0x121AF, 0x1F56F, 0x18BC6, 0x02EB7, 0x16F2E, 0x0AA46, 0x0A26E, 0x18ADA, 0x02315, 0x11900,
    0x111BA, 0x01672, 0x12FAC, 0x00706, 0x1D039, 0x18AAA, 0x02DB5, 0x10AD3, 0x1F78D, 0x000AF,
    0x169E5, 0x02FAC, 0x1886B, 0x1EE9B, 0x0FC3F, 0x0A5B9, 0x0107D, 0x02CEB, 0x1D567, 0x1F036,
    0x00D8A, 0x02ADE, 0x120D5, 0x00840, 0x121D5, 0xE018E, 0x18862, 0x03257, 0x1300F, 0x0A12F,
    0x0FC59, 0x0105E, 0x144BC, 0x027A5, 0x1D163, 0x0A49A, 0x10908, 0x03334, 0x02A2E, 0x0225E,
    0x1B205, 0x0FED2, 0x01CF7, 0x0116D, 0x031CE, 0x18C48, 0x023C4, 0x028F4, 0x0A5F6, 0x033F0,
    0x03014, 0x006B4, 0x0A474, 0x18A63, 0x1B193, 0x0159E, 0x026F9, 0x0A320, 0x00A9D, 0x1D236,
    0x11FCB, 0x10460, 0x00DC2, 0x11D82, 0x14636, 0x01E43, 0x10370, 0x00C38, 0x124C6, 0x1F7B7,
    0x189A6, 0x00849, 0x12123, 0x188CE, 0x018EF, 0x01363, 0x112CA, 0x110A5, 0x00E5B, 0x0A99B,
    0x11221, 0x10C2F, 0x169AC, 0x1AFF5, 0x02A3A, 0x0001F, 0x188C1, 0x13016, 0x122A3, 0x0A69D,
    0x1FB37, 0x1304B, 0x0200D, 0x1B1D8, 0x0A0AE, 0x10889, 0x11DA1, 0x10542, 0x0004E, 0x121D1,
    0x0A1B7, 0x0268C, 0x02FA3, 0x00E2E, 0x02973, 0x02B0E, 0x119BA, 0x0A8A2, 0x12FAF, 0x1D80B,
    0x18C59, 0x011BE, 0x11191, 0x02568, 0x022D6, 0x10672, 0x1E125, 0x0A99F, 0x1D063, 0x1D454,
    0x1D9AD, 0x03070, 0x122F4, 0x0A18F, 0x1D440, 0x000A8, 0x1D5DF, 0x18CA5, 0x16AD3, 0x02B44,
    0x1885B, 0x00912, 0x015ED, 0x1F9C3, 0x024FB, 0x133A5, 0x00118, 0x1014A, 0x1340C, 0x111AD,
    0x02CE2, 0x00801, 0x033D2, 0x1D534, 0x10493, 0x2F92F, 0x01047, 0x1FBC2, 0x1FB13, 0x030D9,
    0x12FBA, 0x1F9B5, 0x00142, 0x00441, 0x16921, 0x0A511, 0x02A5C, 0x01816, 0x2F9CE, 0x013DE,
    0x1B196, 0x0A55D, 0x16F96, 0x1FB2E, 0x2F944, 0x00AA8, 0x144B6, 0x0FC37, 0x019BA, 0x18819,
    0x02EAE, 0x1E8D0, 0x03191, 0x010BA, 0x0A3D3, 0x0273B, 0x18A3E, 0x1FB8C, 0x01784, 0x1F736,
    0x10B8B, 0x0296F, 0x1162F, 0x122DD, 0x1BC4D, 0x115AB, 0x02B50, 0x1FA9E, 0x02130, 0x01EE5,
];

fn bench_get_name(c: &mut Criterion) {
    c.bench_function("get_name", |b| {
        b.iter(|| {
            for ch in TEST_CHARS {
                black_box(get_name(ch));
            }
        })
    });
}

criterion_group!(benches, bench_get_name);
criterion_main!(benches);
