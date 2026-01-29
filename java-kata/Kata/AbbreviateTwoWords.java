package Kata;

public class AbbreviateTwoWords {
    public static String abbrevName(String n) {
        return String.format("%s.%s", n.split(" ")[0].split("")[0],
                n.split(" ")[1].split("")[0]).toUpperCase();
    }
    public static String abbrevName2(String n) {
        String[] a = n.split(" ");
        return String.format("%s.%s", a[0].charAt(0), a[1].charAt(0)).toUpperCase();
    }
}
