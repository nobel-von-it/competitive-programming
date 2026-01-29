package Kata;

import java.util.HashMap;

public class Welcome {

    static HashMap<String, String> langs = new HashMap<>();
    public static void init() {
        langs.put("english", "Welcome");
        langs.put("czech", "Vitejte");
        langs.put("danish", "Velkomst");
        langs.put("dutch", "Welkom");
        langs.put("estonian", "Tere tulemast");
        langs.put("finnish", "Tervetuloa");
        langs.put("flemish", "Welgekomen");
        langs.put("french", "Bienvenue");
        langs.put("german", "Willkommen");
        langs.put("irish", "Failte");
        langs.put("italian", "Benvenuto");
        langs.put("latvian", "Gaidits");
        langs.put("lithuanian", "Laukiamas");
        langs.put("polish", "Witamy");
        langs.put("spanish", "Bienvenido");
        langs.put("swedish", "Valkommen");
        langs.put("welsh", "Croeso");
    }

    public static String greet(String l) {
        init();
        if (langs.containsKey(l)) {
            return langs.get(l);
        }
        return langs.get("english");
    }
}
