@Entity
@Table(name="clients", schema="client")
public class Client {
    @Id @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    private String nom, prenom, adresse, codePostal, ville;
    @Column(name="date_naissance")
    private LocalDate dateNaissance;
    // getters/setters…
}