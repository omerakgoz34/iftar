fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Hakkında
    if args.len() < 2 {
        println!("iftar v0.1.0 by omerakgoz34");
        println!("Kullanım: iftar <şehir>");
        return
    }

    let url_diyanet = "https://namazvakitleri.diyanet.gov.tr/tr-TR/";

    // Şehirler
    let mut cities = std::collections::HashMap::new();
    cities.insert("istanbul", "9541");
    cities.insert("tekirdağ", "9879");
    cities.insert("ordu", "9782");
    cities.insert("adana", "9146");
    cities.insert("adıyaman", "9158");
    cities.insert("afyonkarahisar", "9167");
    cities.insert("ağrı", "9185");
    cities.insert("aksaray", "9193");
    cities.insert("amasya", "9198");
    cities.insert("ankara", "9206");
    cities.insert("antalya", "9225");
    cities.insert("ardahan", "9238");
    cities.insert("artvin", "9246");
    cities.insert("aydın", "9252");
    cities.insert("balıkesir", "9270");
    cities.insert("bartın", "9285");
    cities.insert("batman", "9288");
    cities.insert("bayburt", "9295");
    cities.insert("bilecik", "9297");
    cities.insert("bingöl", "9303");
    cities.insert("bitlis", "9311");
    cities.insert("bolu", "9315");
    cities.insert("burdur", "9327");
    cities.insert("bursa", "9335");
    cities.insert("çanakkale", "9352");
    cities.insert("çankırı", "9359");
    cities.insert("çorum", "9370");
    cities.insert("denizli", "9392");
    cities.insert("diyarbakır", "9402");
    cities.insert("düzce", "9414");
    cities.insert("edirne", "9419");
    cities.insert("elazığ", "9432");
    cities.insert("erzincan", "9440");
    cities.insert("erzurum", "9451");
    cities.insert("eskişehir", "9470");
    cities.insert("gaziantep", "9479");
    cities.insert("giresun", "9494");
    cities.insert("gümüşhane", "9501");
    cities.insert("hakkari", "9507");
    cities.insert("hatay", "20089");
    cities.insert("ığdır", "9522");
    cities.insert("ısparta", "9528");
    cities.insert("izmir", "9560");
    cities.insert("kahramanmaraş", "9577");
    cities.insert("karabük", "9581");
    cities.insert("karaman", "9587");
    cities.insert("kars", "9594");
    cities.insert("kastamonu", "9609");
    cities.insert("kayseri", "9620");
    cities.insert("kilis", "9629");
    cities.insert("kırıkkale", "9635");
    cities.insert("kırklareli", "9638");
    cities.insert("kırşehir", "9646");
    cities.insert("kocaeli", "9654");
    cities.insert("konya", "9676");
    cities.insert("kütahya", "9689");
    cities.insert("malatya", "9703");
    cities.insert("manisa", "9716");
    cities.insert("mardin", "9726");
    cities.insert("mersin", "9737");
    cities.insert("muğla", "9747");
    cities.insert("muş", "9755");
    cities.insert("nevşehir", "9760");
    cities.insert("niğde", "9766");
    cities.insert("osmaniye", "9788");
    cities.insert("rize", "9799");
    cities.insert("sakarya", "9807");
    cities.insert("samsun", "9819");
    cities.insert("şanlıurfa", "9831");
    cities.insert("siirt", "9839");
    cities.insert("sinop", "9847");
    cities.insert("şırnak", "9854");
    cities.insert("sivas", "9868");
    cities.insert("tekirdağ", "9879");
    cities.insert("tokat", "9887");
    cities.insert("trabzon", "9905");
    cities.insert("tunceli", "9914");
    cities.insert("uşak", "9919");
    cities.insert("van", "9930");
    cities.insert("yalova", "9935");
    cities.insert("yozgat", "9949");
    cities.insert("zonguldak", "9955");
    


    let mut city_code = "0";
    match cities.get(&args[1][..]) {
        Some(&city) => city_code = city,
        _ => {
            println!("");
            println!("Girdiğiniz şehir bulunamadı!");
        },
    }
    if city_code == "0" { return }

    let mut url: String = url_diyanet.to_owned();
    url.push_str(city_code);
    let response = minreq::get(url).send().unwrap();

    let fragment = scraper::Html::parse_fragment(response.as_str().unwrap());
    let selector = scraper::Selector::parse("div.tpt-cell[data-vakit-name='aksam'] .tpt-time").unwrap();

    println!("{}: {}", args[1], fragment.select(&selector).next().unwrap().inner_html());
}
