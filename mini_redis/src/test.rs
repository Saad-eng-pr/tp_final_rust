use super::*;

// test basique : est-ce que le serveur répond bien au ping
#[tokio::test]
async fn test_ping() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let resp = process_command("{\"cmd\": \"PING\"}\n", &store).await;
    assert_eq!(resp["status"], "ok");
}

#[tokio::test]
async fn test_set_and_get() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));

    // on set une valeur
    let resp = process_command(
        "{\"cmd\": \"SET\", \"key\": \"foo\", \"value\": \"bar\"}\n",
        &store,
    ).await;
    assert_eq!(resp["status"], "ok");

    // on vérifie qu'on peut la récupérer
    let resp = process_command("{\"cmd\": \"GET\", \"key\": \"foo\"}\n", &store).await;
    assert_eq!(resp["status"], "ok");
    assert_eq!(resp["value"], "bar");

    // get sur une clé qui existe pas
    let resp = process_command("{\"cmd\": \"GET\", \"key\": \"nope\"}\n", &store).await;
    assert!(resp["value"].is_null());
}

#[tokio::test]
async fn test_del() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));

    process_command(
        "{\"cmd\": \"SET\", \"key\": \"k\", \"value\": \"v\"}\n",
        &store,
    ).await;

    let resp = process_command("{\"cmd\": \"DEL\", \"key\": \"k\"}\n", &store).await;
    assert_eq!(resp["count"], 1);

    // re-delete, devrait retourner 0
    let resp = process_command("{\"cmd\": \"DEL\", \"key\": \"k\"}\n", &store).await;
    assert_eq!(resp["count"], 0);
}

#[tokio::test]
async fn test_incr_decr() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));

    // incr sur une clé qui existe pas -> doit créer avec 1
    let resp = process_command("{\"cmd\": \"INCR\", \"key\": \"c\"}\n", &store).await;
    assert_eq!(resp["value"], 1);

    let resp = process_command("{\"cmd\": \"INCR\", \"key\": \"c\"}\n", &store).await;
    assert_eq!(resp["value"], 2);

    let resp = process_command("{\"cmd\": \"DECR\", \"key\": \"c\"}\n", &store).await;
    assert_eq!(resp["value"], 1);
}

#[tokio::test]
async fn test_unknown_command() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let resp = process_command("{\"cmd\": \"FOOBAR\"}\n", &store).await;
    assert_eq!(resp["status"], "error");
}

#[tokio::test]
async fn test_invalid_json() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let resp = process_command("pas du json\n", &store).await;
    assert_eq!(resp["status"], "error");
    assert_eq!(resp["message"], "invalid json");
}
