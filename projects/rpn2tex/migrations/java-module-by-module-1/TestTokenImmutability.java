import com.rpn2tex.*;

public class TestTokenImmutability {
    public static void main(String[] args) {
        Token t = new Token(TokenType.NUMBER, "42", 5, 10);
        
        // Test field access
        System.out.println("Type: " + t.type);
        System.out.println("Value: " + t.value);
        System.out.println("Line: " + t.line);
        System.out.println("Column: " + t.column);
        
        // Try to verify immutability by checking class modifiers
        System.out.println("Class is final: " + java.lang.reflect.Modifier.isFinal(Token.class.getModifiers()));
    }
}
