package Kata;

public class ValidatePIN {
    public static boolean validatePin(String pin) {
        if (pin.length() != 4 && pin.length() != 6) return false;

        String[] a = pin.split("");
        for (String string : a) {
            try {
                int tmp = Integer.parseInt(string);
            } catch (Exception e) {
                return false;
            }
        }
        return true;
    }
    //aaaa
    public static boolean validPin(String pin) {
        return pin.matches("\\d{4}|\\d{6}");
    }
}
