use rs_baseball::game::init_game;
use rs_baseball::game::team::random_team;

#[tokio::test]
async fn it_should_init_the_game() {
    let game = init_game(random_team(), random_team()).await;
    let home_team = game.home_team;
    assert_eq!(home_team.players.len(), 9);
    assert_eq!(game.half_inning, 0);
    assert_eq!(game.outs, 0);
}

#[tokio::test]
async fn it_should_start_the_game() {
    let mut game = init_game(random_team(), random_team()).await;
    let mut rx = game.game_channel_tx.subscribe();
    println!("Spawning monitoring listener");
    tokio::spawn(async move {
        loop {
            let message = rx.recv().await.unwrap();
            println!("{:?}", message.message_type);
        }
    });
    tokio::spawn(async move {
        game.start().await;
    }).await.unwrap();
}
