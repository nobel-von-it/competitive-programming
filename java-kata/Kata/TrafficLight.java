package Kata;

public class TrafficLight {
    public static String updateLight(String curr) {
        String[] all = "green,yellow,red,green".split(",");
        for (int i = 0; i < all.length; i++) {
            if (curr.equals(all[i])) {
                return all[i+1];
            }
        }
        return "";
    }
    // or switch
    public static String upLight(String curr) {
        switch (curr) {
            case "red": return "green";
            case "yellow": return "red";
            case "green": return "yellow";
            default: throw new IllegalArgumentException();
        }
    }
}
