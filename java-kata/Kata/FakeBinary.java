package Kata;

public class FakeBinary {
    public static String fakeBin(String n) {
        return n.replaceAll("[0-4]", "0").replaceAll("[5-9]", "1");
    }
}
