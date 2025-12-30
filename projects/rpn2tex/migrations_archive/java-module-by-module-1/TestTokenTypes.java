import com.rpn2tex.*;

public class TestTokenTypes {
    public static void main(String[] args) {
        System.out.println("TokenType enum values:");
        for (TokenType type : TokenType.values()) {
            System.out.println("  - " + type.name());
        }
    }
}
