public class NewClient {
    @NotBlank private String nom, prenom, adresse, codePostal, ville;
    @NotNull @DateTimeFormat(pattern="yyyy-MM-dd")
    private LocalDate dateNaissance;
    // getters/setters…
}