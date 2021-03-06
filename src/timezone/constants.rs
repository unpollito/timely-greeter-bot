pub const GREET_AT_HOUR: u32 = 9;

pub const TIMEZONES: [chrono_tz::Tz; 178] = [
    chrono_tz::Africa::Accra,
    chrono_tz::Africa::Algiers,
    chrono_tz::Africa::Bissau,
    chrono_tz::Africa::Cairo,
    chrono_tz::Africa::Casablanca,
    chrono_tz::Africa::El_Aaiun,
    chrono_tz::Africa::Johannesburg,
    chrono_tz::Africa::Khartoum,
    chrono_tz::Africa::Lagos,
    chrono_tz::Africa::Monrovia,
    chrono_tz::Africa::Ndjamena,
    chrono_tz::Africa::Nairobi,
    chrono_tz::Africa::Tripoli,
    chrono_tz::Africa::Tunis,
    chrono_tz::Africa::Windhoek,
    chrono_tz::America::Adak,
    chrono_tz::America::Anchorage,
    chrono_tz::America::Argentina::Buenos_Aires,
    chrono_tz::America::Argentina::Cordoba,
    chrono_tz::America::Asuncion,
    chrono_tz::America::Barbados,
    chrono_tz::America::Belize,
    chrono_tz::America::Bogota,
    chrono_tz::America::Cancun,
    chrono_tz::America::Caracas,
    chrono_tz::America::Cayenne,
    chrono_tz::America::Chicago,
    chrono_tz::America::Chihuahua,
    chrono_tz::America::Costa_Rica,
    chrono_tz::America::Curacao,
    chrono_tz::America::Danmarkshavn,
    chrono_tz::America::Denver,
    chrono_tz::America::Detroit,
    chrono_tz::America::El_Salvador,
    chrono_tz::America::Godthab,
    chrono_tz::America::Guatemala,
    chrono_tz::America::Guayaquil,
    chrono_tz::America::Guyana,
    chrono_tz::America::Havana,
    chrono_tz::America::Indiana::Indianapolis,
    chrono_tz::America::Jamaica,
    chrono_tz::America::Juneau,
    chrono_tz::America::La_Paz,
    chrono_tz::America::Lima,
    chrono_tz::America::Los_Angeles,
    chrono_tz::America::Managua,
    chrono_tz::America::Martinique,
    chrono_tz::America::Menominee,
    chrono_tz::America::Metlakatla,
    chrono_tz::America::Mexico_City,
    chrono_tz::America::Miquelon,
    chrono_tz::America::Montevideo,
    chrono_tz::America::Nassau,
    chrono_tz::America::New_York,
    chrono_tz::America::Nome,
    chrono_tz::America::Noronha,
    chrono_tz::America::Panama,
    chrono_tz::America::Paramaribo,
    chrono_tz::America::Phoenix,
    chrono_tz::America::Port_of_Spain,
    chrono_tz::America::PortauPrince,
    chrono_tz::America::Porto_Velho,
    chrono_tz::America::Puerto_Rico,
    chrono_tz::America::Santiago,
    chrono_tz::America::Santo_Domingo,
    chrono_tz::America::Sao_Paulo,
    chrono_tz::America::Sitka,
    chrono_tz::America::St_Johns,
    chrono_tz::America::Tegucigalpa,
    chrono_tz::America::Tijuana,
    chrono_tz::America::Toronto,
    chrono_tz::America::Vancouver,
    chrono_tz::America::Yakutat,
    chrono_tz::Antarctica::Casey,
    chrono_tz::Antarctica::Davis,
    chrono_tz::Antarctica::DumontDUrville,
    chrono_tz::Antarctica::Mawson,
    chrono_tz::Antarctica::Vostok,
    chrono_tz::Asia::Aqtobe,
    chrono_tz::Asia::Almaty,
    chrono_tz::Asia::Baghdad,
    chrono_tz::Asia::Baku,
    chrono_tz::Asia::Bangkok,
    chrono_tz::Asia::Barnaul,
    chrono_tz::Asia::Beirut,
    chrono_tz::Asia::Brunei,
    chrono_tz::Asia::Chita,
    chrono_tz::Asia::Colombo,
    chrono_tz::Asia::Dhaka,
    chrono_tz::Asia::Ho_Chi_Minh,
    chrono_tz::Asia::Hong_Kong,
    chrono_tz::Asia::Irkutsk,
    chrono_tz::Asia::Jakarta,
    chrono_tz::Asia::Jayapura,
    chrono_tz::Asia::Jerusalem,
    chrono_tz::Asia::Kabul,
    chrono_tz::Asia::Kamchatka,
    chrono_tz::Asia::Karachi,
    chrono_tz::Asia::Kathmandu,
    chrono_tz::Asia::Khandyga,
    chrono_tz::Asia::Kolkata,
    chrono_tz::Asia::Kuala_Lumpur,
    chrono_tz::Asia::Makassar,
    chrono_tz::Asia::Manila,
    chrono_tz::Asia::Nicosia,
    chrono_tz::Asia::Novosibirsk,
    chrono_tz::Asia::Omsk,
    chrono_tz::Asia::Pyongyang,
    chrono_tz::Asia::Qatar,
    chrono_tz::Asia::Sakhalin,
    chrono_tz::Asia::Seoul,
    chrono_tz::Asia::Shanghai,
    chrono_tz::Asia::Taipei,
    chrono_tz::Asia::Tbilisi,
    chrono_tz::Asia::Tehran,
    chrono_tz::Asia::Tokyo,
    chrono_tz::Asia::Vladivostok,
    chrono_tz::Asia::Yangon,
    chrono_tz::Asia::Yakutsk,
    chrono_tz::Asia::Yekaterinburg,
    chrono_tz::Asia::Yerevan,
    chrono_tz::Atlantic::Azores,
    chrono_tz::Atlantic::Bermuda,
    chrono_tz::Atlantic::Canary,
    chrono_tz::Atlantic::Cape_Verde,
    chrono_tz::Atlantic::Faroe,
    chrono_tz::Atlantic::Madeira,
    chrono_tz::Atlantic::Reykjavik,
    chrono_tz::Atlantic::South_Georgia,
    chrono_tz::Australia::Adelaide,
    chrono_tz::Australia::Broken_Hill,
    chrono_tz::Australia::Currie,
    chrono_tz::Australia::Darwin,
    chrono_tz::Australia::Hobart,
    chrono_tz::Australia::Lord_Howe,
    chrono_tz::Australia::Melbourne,
    chrono_tz::Australia::Sydney,
    chrono_tz::Europe::Amsterdam,
    chrono_tz::Europe::Astrakhan,
    chrono_tz::Europe::Berlin,
    chrono_tz::Europe::Bucharest,
    chrono_tz::Europe::Copenhagen,
    chrono_tz::Europe::Gibraltar,
    chrono_tz::Europe::Istanbul,
    chrono_tz::Europe::Kiev,
    chrono_tz::Europe::Lisbon,
    chrono_tz::Europe::London,
    chrono_tz::Europe::Luxembourg,
    chrono_tz::Europe::Oslo,
    chrono_tz::Europe::Paris,
    chrono_tz::Europe::Moscow,
    chrono_tz::Europe::Simferopol,
    chrono_tz::Europe::Stockholm,
    chrono_tz::Indian::Christmas,
    chrono_tz::Indian::Cocos,
    chrono_tz::Indian::Kerguelen,
    chrono_tz::Indian::Mahe,
    chrono_tz::Indian::Maldives,
    chrono_tz::Indian::Mauritius,
    chrono_tz::Indian::Reunion,
    chrono_tz::Pacific::Apia,
    chrono_tz::Pacific::Auckland,
    chrono_tz::Pacific::Chatham,
    chrono_tz::Pacific::Easter,
    chrono_tz::Pacific::Efate,
    chrono_tz::Pacific::Enderbury,
    chrono_tz::Pacific::Fiji,
    chrono_tz::Pacific::Gambier,
    chrono_tz::Pacific::Galapagos,
    chrono_tz::Pacific::Guam,
    chrono_tz::Pacific::Honolulu,
    chrono_tz::Pacific::Kiritimati,
    chrono_tz::Pacific::Marquesas,
    chrono_tz::Pacific::Noumea,
    chrono_tz::Pacific::Pitcairn,
    chrono_tz::Pacific::Rarotonga,
    chrono_tz::Pacific::Tahiti,
    chrono_tz::Pacific::Tarawa,
];
