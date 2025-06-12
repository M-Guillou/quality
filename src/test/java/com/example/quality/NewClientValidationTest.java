package com.example.quality;

import com.example.quality.dto.NewClient;
import jakarta.validation.ConstraintViolation;
import jakarta.validation.Validation;
import jakarta.validation.Validator;
import jakarta.validation.ValidatorFactory;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.time.LocalDate;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

public class NewClientValidationTest {

    private static ValidatorFactory validatorFactory;
    private static Validator validator;

    @BeforeAll
    public static void setUp() {
        validatorFactory = Validation.buildDefaultValidatorFactory();
        validator = validatorFactory.getValidator();
    }

    @AfterAll
    public static void tearDown() {
        validatorFactory.close();
    }

    @Test
    void validNewClient_ShouldHaveNoViolations() {
        NewClient client = new NewClient();
        client.setNom("Doe");
        client.setPrenom("John");
        client.setAdresse("123 rue Exemple");
        client.setCodePostal("75000");
        client.setVille("Paris");
        client.setDateNaissance(LocalDate.of(1990, 1, 1));

        Set<ConstraintViolation<NewClient>> violations = validator.validate(client);
        assertTrue(violations.isEmpty(), "Valid client should have no validation errors");
    }

    @Test
    void invalidNewClient_MissingFields_ShouldHaveViolations() {
        NewClient client = new NewClient();
        // All fields empty or null

        Set<ConstraintViolation<NewClient>> violations = validator.validate(client);
        assertFalse(violations.isEmpty(), "Empty client should have validation errors");

        // Check that specific fields cause violations
        boolean nomViolation = violations.stream()
                .anyMatch(v -> v.getPropertyPath().toString().equals("nom"));
        assertTrue(nomViolation, "Missing nom should cause violation");

        boolean dateNaissanceViolation = violations.stream()
                .anyMatch(v -> v.getPropertyPath().toString().equals("dateNaissance"));
        assertTrue(dateNaissanceViolation, "Missing dateNaissance should cause violation");
    }
}
