use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

static HARVEY_COVER: Asset = asset!("/assets/harvey_cover.jpg");
static STRESS_COVER: Asset = asset!("/assets/stress_relief_cover.jpg");

static HARVEY_AUDIO: Asset = asset!("/assets/harvey.mp3");
static STRESS_AUDIO: Asset = asset!("/assets/stress_relief.mp3");

const CLIP_DURATION: f32 = 23.0;
const STRESS_START_DELAY: f32 = 8.65;
const TOTAL_DURATION: f32 = CLIP_DURATION + STRESS_START_DELAY;
const END_EARLY_SECONDS: f32 = 2.5;
const PLAYBACK_DURATION: f32 = TOTAL_DURATION - END_EARLY_SECONDS;
const HARVEY_VOLUME: f32 = 0.12;
const STRESS_VOLUME: f32 = 0.11;

const SHARED_AURA_BEAT_SECONDS: f32 = 0.851;

#[derive(Clone, Copy, PartialEq)]
struct Cue {
    at: f32,
    text: &'static str,
}

static HARVEY_CUES: &[Cue] = &[
    Cue {
        at: 1.5,
        text: "Harvey",
    },
    Cue {
        at: 3.0,
        text: "Nobody knows what I see",
    },
    Cue {
        at: 6.5,
        text: "Nobody knows I'm waiting",
    },
    Cue {
        at: 10.0,
        text: "Waiting for you to call",
    },
    Cue {
        at: 15.0,
        text: "Harvey",
    },
    Cue {
        at: 16.5,
        text: "Nobody knows what I see",
    },
    Cue {
        at: 20.2,
        text: "Everyone thinks I'm crazy",
    },
    Cue {
        at: 23.5,
        text: "Crazy for you, oh boy",
    },
];

static STRESS_CUES: &[Cue] = &[
    Cue {
        at: 0.0,
        text: "It's stress relief from everything",
    },
    Cue {
        at: 6.5,
        text: "Tell me, tell me you love me",
    },
    Cue {
        at: 10.0,
        text: "Come back, come back to haunt me",
    },
    Cue {
        at: 13.5,
        text: "Won't you...",
    },
    Cue {
        at: 15.0,
        text: "won't you let me be myself?",
    },
];

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut elapsed = use_signal(|| 0.0_f32);
    let mut playing = use_signal(|| false);
    let mut has_started = use_signal(|| false);

    let start = move |_| {
        println!("[ui] START / RESTART clicked");
        println!("[asset] CSS = {CSS}");
        println!("[asset] Harvey cover = {HARVEY_COVER}");
        println!("[asset] Stress cover = {STRESS_COVER}");
        println!("[asset] Harvey audio = {HARVEY_AUDIO}");
        println!("[asset] Stress audio = {STRESS_AUDIO}");

        elapsed.set(0.0);
        playing.set(true);
        has_started.set(true);

        let stress_delay_ms = (STRESS_START_DELAY * 1000.0) as u32;

        let script = format!(
            r#"
            (async () => {{
                try {{
                    if (window.__stressHarveyTimer) {{
                        clearInterval(window.__stressHarveyTimer);
                        window.__stressHarveyTimer = null;
                    }}

                    if (window.__stressHarveyDelay) {{
                        clearTimeout(window.__stressHarveyDelay);
                        window.__stressHarveyDelay = null;
                    }}

                    const harvey = document.getElementById("harvey-audio");
                    const stress = document.getElementById("stress-audio");

                    if (!harvey) {{
                        dioxus.send("ERROR: harvey-audio element not found");
                        return;
                    }}

                    if (!stress) {{
                        dioxus.send("ERROR: stress-audio element not found");
                        return;
                    }}

                    harvey.pause();
                    stress.pause();

                    harvey.currentTime = 0;
                    stress.currentTime = 0;

                    harvey.volume = {harvey_volume};
                    stress.volume = {stress_volume};

                    harvey.load();
                    stress.load();

                    const debug = {{
                        harveySrc: harvey.currentSrc || harvey.src,
                        stressSrc: stress.currentSrc || stress.src,
                        canMp3: harvey.canPlayType("audio/mpeg"),
                        harveyNetworkState: harvey.networkState,
                        harveyReadyState: harvey.readyState,
                        harveyError: harvey.error ? {{
                            code: harvey.error.code,
                            message: harvey.error.message
                        }} : null,
                        stressDelaySeconds: {stress_delay_seconds},
                        totalDurationSeconds: {total_duration}
                    }};

                    dioxus.send(JSON.stringify(debug));

                    const startedAt = performance.now();

                    await harvey.play();
                    dioxus.send("OK: Harvey started");

                    window.__stressHarveyDelay = setTimeout(async () => {{
                        try {{
                            stress.currentTime = 0;
                            await stress.play();
                            dioxus.send("OK: Stress Relief started after 6 seconds");
                        }} catch (err) {{
                            dioxus.send("STRESS JS ERROR: " + err.name + " - " + err.message);
                        }}
                    }}, {stress_delay_ms});

                    window.__stressHarveyTimer = setInterval(() => {{
                        const t = (performance.now() - startedAt) / 1000.0;

                        dioxus.send("TIME:" + t.toFixed(3));

                        if (t >= {total_duration}) {{
                            clearInterval(window.__stressHarveyTimer);
                            window.__stressHarveyTimer = null;

                            if (window.__stressHarveyDelay) {{
                                clearTimeout(window.__stressHarveyDelay);
                                window.__stressHarveyDelay = null;
                            }}

                            harvey.pause();
                            stress.pause();

                            dioxus.send("END");
                        }}
                    }}, 40);
                }} catch (err) {{
                    dioxus.send("JS ERROR: " + err.name + " - " + err.message);
                }}
            }})();
            "#,
            harvey_volume = HARVEY_VOLUME,
            stress_volume = STRESS_VOLUME,
            stress_delay_ms = stress_delay_ms,
            stress_delay_seconds = STRESS_START_DELAY,
            total_duration = PLAYBACK_DURATION,
        );

        let mut eval = document::eval(&script);

        spawn(async move {
            while let Ok(message) = eval.recv::<String>().await {
                if let Some(raw_time) = message.strip_prefix("TIME:") {
                    if let Ok(seconds) = raw_time.parse::<f32>() {
                        elapsed.set(seconds.clamp(0.0, PLAYBACK_DURATION));
                    }
                    continue;
                }

                if message == "END" {
                    println!("[timer] clip ended");
                    elapsed.set(PLAYBACK_DURATION);
                    playing.set(false);
                    continue;
                }

                println!("[webview] {message}");
            }
        });
    };

    let pause = move |_| {
        println!("[ui] PAUSE clicked");

        playing.set(false);

        document::eval(
            r#"
            if (window.__stressHarveyTimer) {
                clearInterval(window.__stressHarveyTimer);
                window.__stressHarveyTimer = null;
            }

            if (window.__stressHarveyDelay) {
                clearTimeout(window.__stressHarveyDelay);
                window.__stressHarveyDelay = null;
            }

            document.getElementById("harvey-audio")?.pause();
            document.getElementById("stress-audio")?.pause();
            "#,
        );
    };

    let time = *elapsed.read();
    let is_playing = *playing.read();

    let bar_time = time;

    let harvey_lyric_time = time;
    let stress_lyric_time = delayed_elapsed(time, STRESS_START_DELAY);

    let aura_is_animating = is_playing && time > 0.0 && time < PLAYBACK_DURATION;

    let harvey_aura_visible = aura_is_animating;
    let stress_aura_visible = is_playing && time >= STRESS_START_DELAY && time < PLAYBACK_DURATION;

    rsx! {
        document::Stylesheet { href: CSS }

        main { class: "page",
            audio {
                id: "harvey-audio",
                preload: "auto",

                source {
                    src: "{HARVEY_AUDIO}",
                    r#type: "audio/mpeg"
                }
            }

            audio {
                id: "stress-audio",
                preload: "auto",

                source {
                    src: "{STRESS_AUDIO}",
                    r#type: "audio/mpeg"
                }
            }

            if *has_started.read() {
                div { class: "floating-cards",
                    MusicCard {
                        title: "Harvey",
                        cover: HARVEY_COVER,
                        cues: HARVEY_CUES,
                        bar_elapsed: bar_time,
                        lyric_elapsed: harvey_lyric_time,
                        duration: PLAYBACK_DURATION,
                        aura_visible: harvey_aura_visible,
                        aura_animating: aura_is_animating,
                        beat_seconds: SHARED_AURA_BEAT_SECONDS,
                        aura_rgb: "91, 137, 109",
                        aura_phase_class: "aura-phase-a"
                    }

                    MusicCard {
                        title: "Stress Relief",
                        cover: STRESS_COVER,
                        cues: STRESS_CUES,
                        bar_elapsed: bar_time,
                        lyric_elapsed: stress_lyric_time,
                        duration: PLAYBACK_DURATION,
                        aura_visible: stress_aura_visible,
                        aura_animating: aura_is_animating,
                        beat_seconds: SHARED_AURA_BEAT_SECONDS,
                        aura_rgb: "239, 127, 119",
                        aura_phase_class: "aura-phase-b"
                    }
                }
            }

            div { class: "floating-controls",
                button {
                    class: "control-button",
                    onclick: start,
                    "Start / Restart"
                }

                button {
                    class: "control-button secondary",
                    onclick: pause,
                    "Pause"
                }
            }
        }
    }
}

#[component]
fn MusicCard(
    title: &'static str,
    cover: Asset,
    cues: &'static [Cue],
    bar_elapsed: f32,
    lyric_elapsed: f32,
    duration: f32,
    aura_visible: bool,
    aura_animating: bool,
    beat_seconds: f32,
    aura_rgb: &'static str,
    aura_phase_class: &'static str,
) -> Element {
    let safe_bar_elapsed = bar_elapsed.max(0.0);
    let progress = ((safe_bar_elapsed / duration) * 100.0).clamp(0.0, 100.0);

    let lyric = current_text(cues, lyric_elapsed);

    let aura_visibility_class = if aura_visible { "visible" } else { "idle" };
    let aura_play_state = if aura_animating { "running" } else { "paused" };

    rsx! {
        section { class: "card",
            h2 { class: "song-title", "{title}" }

            div {
                class: "cover-stage",
                style: "--beat-duration: {beat_seconds}s; --aura-rgb: {aura_rgb}; --aura-play-state: {aura_play_state};",

                div {
                    class: "aura-fade {aura_visibility_class}",

                    div {
                        class: "cover-aura breathing {aura_phase_class}"
                    }
                }

                img {
                    class: "cover",
                    src: "{cover}",
                    alt: "{title} cover"
                }
            }

            div { class: "progress-track",
                div {
                    class: "progress-fill",
                    style: "width: {progress}%;"
                }

                div {
                    class: "progress-dot",
                    style: "left: {progress}%;"
                }
            }

            p { class: "lyric", "{lyric}" }
        }
    }
}

fn current_text(cues: &[Cue], elapsed: f32) -> &'static str {
    let mut result = "";

    for cue in cues {
        if elapsed >= cue.at {
            result = cue.text;
        } else {
            break;
        }
    }

    result
}

fn delayed_elapsed(global_elapsed: f32, delay: f32) -> f32 {
    if global_elapsed < delay {
        -1.0
    } else {
        global_elapsed - delay
    }
}
