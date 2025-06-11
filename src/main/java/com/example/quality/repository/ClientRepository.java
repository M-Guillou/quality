@Repository
public interface ClientRepository extends JpaRepository<Client, Long> {
    List<Client> findAllByOrderByIdDesc();
}