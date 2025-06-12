package com.example.quality.dto;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;
import lombok.Data;
import org.springframework.format.annotation.DateTimeFormat;

import java.time.LocalDate;

@Data
public class NewClient {
    @NotBlank
    private String nom, prenom, adresse, codePostal, ville;
    @NotNull
    @DateTimeFormat(pattern="yyyy-MM-dd")
    private LocalDate dateNaissance;
}