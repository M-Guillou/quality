package com.example.quality.model;

import jakarta.persistence.*;
import lombok.Data;

import java.util.Date;

@Data
@Entity
@Table(name="clients", schema="client")
public class Client {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    private String nom, prenom, adresse, codePostal, ville;
    @Column(name="date_naissance")
    private Date dateNaissance;
    // getters/setters…
}