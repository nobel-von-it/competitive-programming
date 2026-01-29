package Kata;

public class Stringy {
    public static String stringy(int size) {
//        StringBuilder stringBuilder = new StringBuilder();
        String str = "";
        for (int i = 0; i < size; i++) {
            if (i % 2 == 0) {
//                stringBuilder.append("1");
                str += "1";
            } else {
//                stringBuilder.append("0");
                str += "0";
            }
        }
        return str;
        // or return stringBuilder.toString();
    }
}
