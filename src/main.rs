use chrono::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Hakkında
    if args.len() < 2 {
        println!("");
        println!("* iftar v{} by omerakgoz34", env!("CARGO_PKG_VERSION"));
        println!("* Kullanım: iftar <şehir>");
        println!("* NOT: İftar vakti her gece 00:00'dan sonra yenilenir.");
        println!("* Hata bildirimi ve sorular için: https://github.com/omerakgoz34/iftar/issues/new");
        return
    }

    let url_diyanet = "https://namazvakitleri.diyanet.gov.tr/tr-TR/";

    // Şehirler
    let mut cities = std::collections::HashMap::new();
    cities.insert("istanbul", "9541/istanbul-icin-namaz-vakti");
    cities.insert("tekirdağ", "9879/tekirdag-icin-namaz-vakti");
    cities.insert("ordu", "9782/ordu-icin-namaz-vakti");
    cities.insert("ünye", "9783/unye-icin-namaz-vakti");
    cities.insert("ikizce", "9777/ikizce-icin-namaz-vakti");
    cities.insert("adana", "9146/adana-icin-namaz-vakti");
    cities.insert("adıyaman", "9158/adiyaman-icin-namaz-vakti");
    cities.insert("afyonkarahisar", "9167/afyonkarahisar-icin-namaz-vakti");
    cities.insert("ağrı", "9185/agri-icin-namaz-vakti");
    cities.insert("aksaray", "9193/aksaray-icin-namaz-vakti");
    cities.insert("amasya", "9198/amasya-icin-namaz-vakti");
    cities.insert("ankara", "9206/ankara-icin-namaz-vakti");
    cities.insert("antalya", "9225/antalya-icin-namaz-vakti");
    cities.insert("ardahan", "9238/ardahan-icin-namaz-vakti");
    cities.insert("artvin", "9246/artvin-icin-namaz-vakti");
    cities.insert("aydın", "9252/aydin-icin-namaz-vakti");
    cities.insert("balıkesir", "9270/balikesir-icin-namaz-vakti");
    cities.insert("bartın", "9285/bartin-icin-namaz-vakti");
    cities.insert("batman", "9288/batman-icin-namaz-vakti");
    cities.insert("bayburt", "9295/bayburt-icin-namaz-vakti");
    cities.insert("bilecik", "9297/bilecik-icin-namaz-vakti");
    cities.insert("bingöl", "9303/bingol-icin-namaz-vakti");
    cities.insert("bitlis", "9311/bitlis-icin-namaz-vakti");
    cities.insert("bolu", "9315/bolu-icin-namaz-vakti");
    cities.insert("burdur", "9327/burdur-icin-namaz-vakti");
    cities.insert("bursa", "9335/bursa-icin-namaz-vakti");
    cities.insert("çanakkale", "9352/canakkale-icin-namaz-vakti");
    cities.insert("çankırı", "9359/cankiri-icin-namaz-vakti");
    cities.insert("çorum", "9370/corum-icin-namaz-vakti");
    cities.insert("denizli", "9392/denizli-icin-namaz-vakti");
    cities.insert("diyarbakır", "9402/diyarbakir-icin-namaz-vakti");
    cities.insert("düzce", "9414/duzce-icin-namaz-vakti");
    cities.insert("edirne", "9419/edirne-icin-namaz-vakti");
    cities.insert("elazığ", "9432/elazig-icin-namaz-vakti");
    cities.insert("erzincan", "9440/erzincan-icin-namaz-vakti");
    cities.insert("erzurum", "9451/erzurum-icin-namaz-vakti");
    cities.insert("eskişehir", "9470/eskisehir-icin-namaz-vakti");
    cities.insert("gaziantep", "9479/gaziantep-icin-namaz-vakti");
    cities.insert("giresun", "9494/giresun-icin-namaz-vakti");
    cities.insert("gümüşhane", "9501/gumushane-icin-namaz-vakti");
    cities.insert("hakkari", "9507/hakkari-icin-namaz-vakti");
    cities.insert("hatay", "20089/hatay-icin-namaz-vakti");
    cities.insert("ığdır", "9522/igdir-icin-namaz-vakti");
    cities.insert("ısparta", "9528/isparta-icin-namaz-vakti");
    cities.insert("izmir", "9560/izmir-icin-namaz-vakti");
    cities.insert("kahramanmaraş", "9577/kahramanmaras-icin-namaz-vakti");
    cities.insert("karabük", "9581/karabuk-icin-namaz-vakti");
    cities.insert("karaman", "9587/karaman-icin-namaz-vakti");
    cities.insert("kars", "9594/kars-icin-namaz-vakti");
    cities.insert("kastamonu", "9609/kastamonu-icin-namaz-vakti");
    cities.insert("kayseri", "9620/kayseri-icin-namaz-vakti");
    cities.insert("kilis", "9629/kilis-icin-namaz-vakti");
    cities.insert("kırıkkale", "9635/kirikkale-icin-namaz-vakti");
    cities.insert("kırklareli", "9638/kirklareli-icin-namaz-vakti");
    cities.insert("kırşehir", "9646/kirsehir-icin-namaz-vakti");
    cities.insert("kocaeli", "9654/kocaeli-icin-namaz-vakti");
    cities.insert("konya", "9676/konya-icin-namaz-vakti");
    cities.insert("kütahya", "9689/kutahya-icin-namaz-vakti");
    cities.insert("malatya", "9703/malatya-icin-namaz-vakti");
    cities.insert("manisa", "9716/manisa-icin-namaz-vakti");
    cities.insert("mardin", "9726/mardin-icin-namaz-vakti");
    cities.insert("mersin", "9737/mersin-icin-namaz-vakti");
    cities.insert("muğla", "9747/mugla-icin-namaz-vakti");
    cities.insert("muş", "9755/mus-icin-namaz-vakti");
    cities.insert("nevşehir", "9760/nevsehir-icin-namaz-vakti");
    cities.insert("niğde", "9766/nigde-icin-namaz-vakti");
    cities.insert("osmaniye", "9788/osmaniye-icin-namaz-vakti");
    cities.insert("rize", "9799/rize-icin-namaz-vakti");
    cities.insert("sakarya", "9807/sakarya-icin-namaz-vakti");
    cities.insert("samsun", "9819/samsun-icin-namaz-vakti");
    cities.insert("şanlıurfa", "9831/sanliurfa-icin-namaz-vakti");
    cities.insert("siirt", "9839/siirt-icin-namaz-vakti");
    cities.insert("sinop", "9847/sinop-icin-namaz-vakti");
    cities.insert("şırnak", "9854/sirnak-icin-namaz-vakti");
    cities.insert("sivas", "9868/sivas-icin-namaz-vakti");
    cities.insert("tekirdağ", "9879/tekirdag-icin-namaz-vakti");
    cities.insert("tokat", "9887/tokat-icin-namaz-vakti");
    cities.insert("trabzon", "9905/trabzon-icin-namaz-vakti");
    cities.insert("tunceli", "9914/tunceli-icin-namaz-vakti");
    cities.insert("uşak", "9919/usak-icin-namaz-vakti");
    cities.insert("van", "9930/van-icin-namaz-vakti");
    cities.insert("yalova", "9935/yalova-icin-namaz-vakti");
    cities.insert("yozgat", "9949/yozgat-icin-namaz-vakti");
    cities.insert("zonguldak", "9955/zonguldak-icin-namaz-vakti");
    
    // Şehir kodu
    let city = args[1].to_owned();
    let city_code = match cities.get(&city[..]) {
        Some(&res) => res,
        _ => {
            println!("");
            println!(">>> HATA: Girdiğiniz şehir bulunamadı! İsimleri bitişik ve küçük harflerle yazınız.");
            return;
        },
    };

    // Final URL
    let url = &(url_diyanet.to_owned() + city_code)[..];
    let response = match  minreq::get(url).send() {
        Ok(res) => res,
        Err(_) => {
            println!("");
            println!(">>> HATA: Sunucuya bağlanılamadı! İnternete bağlı olduğunuzdan emin olunuz.");
            return;
        },
    };

    // Sunucudan gelen yanıtı yazıya çevirme
    let response = match response.as_str() {
        Ok(res) => res,
        Err(_) => {
            println!("");
            println!(">>> HATA: Sunucudan alınan veri okunamadı! (UTF-8 hatası)");
            return;
        },
    };

    // CSS selector kullanarak iftar saati bilgisini bulma
    let fragment = scraper::Html::parse_fragment(response);
    let selector = match scraper::Selector::parse("div.tpt-cell[data-vakit-name='aksam'] .tpt-time") {
        Ok(res) => res,
        Err(_) => {
            println!("");
            println!(">>> HATA: İftar saati bulunamadı! Tekrar deneyiniz.");
            return;
        },
    };
    let iftar = fragment.select(&selector).next().unwrap().inner_html();

    // Zaman hesaplamaları
    let now = Local::now();
    let iftar_split: Vec<&str> = iftar.split(":").collect();
    let iftar_time = Local.ymd(now.year(), now.month(), now.day()).and_hms(iftar_split[0].parse::<u32>().unwrap(), iftar_split[1].parse::<u32>().unwrap(), 0);
    
    #[cfg(debug_assertions)]
    {
        println!("iftar_time: {}", iftar_time);
        println!("now: {}", now);
        println!("kalan: {}", Local.timestamp(iftar_time.timestamp() - now.timestamp(), 0));
        println!("geçen: {}", Local.timestamp(now.timestamp() - iftar_time.timestamp(), 0));
    }

    // İftara daha vakit var
    if now < iftar_time {
        let remaining_time = Local.timestamp(iftar_time.timestamp() - now.timestamp(), 0);
        println!("");
        println!("* Şehir: {}", city);
        println!("* İftar vakti: {}", iftar);
        println!("* Kalan süre: {} saat {} dakika", remaining_time.hour() - 2, remaining_time.minute() + 1);
    }
    // İftar geçti
    if now > iftar_time {
        println!("");
        println!("* Şehir: {}", city);
        println!("* İftar vakti: {} (geçti)", iftar);
    }
}
