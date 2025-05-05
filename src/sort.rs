use std::time::Duration;
use tokio::sync::mpsc;

use crate::app::{SortEvent, SortMessage};

pub async fn bubble_sort(
    mut array: Vec<u32>,
    event_tx: mpsc::Sender<SortEvent>,
    sort_rx: &mut mpsc::Receiver<SortMessage>,
) {
    let len = array.len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            if let Ok(msg) = sort_rx.try_recv() {
                match msg {
                    SortMessage::Pause => {
                        let _ = event_tx.send(SortEvent::Comparing(j, j + 1)).await;

                        loop {
                            match tokio::time::timeout(Duration::from_millis(100), sort_rx.recv())
                                .await
                            {
                                Ok(Some(SortMessage::Continue)) => break,
                                Ok(Some(SortMessage::Stop)) => return,
                                Ok(_) => {}
                                Err(_) => {}
                            }
                        }
                    }
                    SortMessage::Stop => return,
                    _ => {}
                }
            }

            let _ = event_tx.send(SortEvent::Comparing(j, j + 1)).await;
            let _ = event_tx.send(SortEvent::StepIncrement).await;
            tokio::time::sleep(Duration::from_millis(300)).await;

            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                let _ = event_tx.send(SortEvent::Swapping(j, j + 1)).await;
            }
        }
    }

    let _ = event_tx.send(SortEvent::Done).await;
}

pub async fn selection_sort(
    mut array: Vec<u32>,
    event_tx: mpsc::Sender<SortEvent>,
    sort_rx: &mut mpsc::Receiver<SortMessage>,
) {
    let len = array.len();

    for i in 0..len {
        let mut min_idx = i;

        for j in i + 1..len {
            if let Ok(msg) = sort_rx.try_recv() {
                match msg {
                    SortMessage::Pause => {
                        let _ = event_tx.send(SortEvent::Comparing(min_idx, j)).await;

                        loop {
                            match tokio::time::timeout(Duration::from_millis(100), sort_rx.recv())
                                .await
                            {
                                Ok(Some(SortMessage::Continue)) => break,
                                Ok(Some(SortMessage::Stop)) => return,
                                Ok(_) => {}
                                Err(_) => {}
                            }
                        }
                    }
                    SortMessage::Stop => return,
                    _ => {}
                }
            }

            let _ = event_tx.send(SortEvent::Comparing(min_idx, j)).await;
            let _ = event_tx.send(SortEvent::StepIncrement).await;
            tokio::time::sleep(Duration::from_millis(300)).await;

            if array[j] < array[min_idx] {
                min_idx = j;
            }
        }

        if min_idx != i {
            array.swap(i, min_idx);
            let _ = event_tx.send(SortEvent::Swapping(i, min_idx)).await;
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    }

    let _ = event_tx.send(SortEvent::Done).await;
}

pub async fn insertion_sort(
    mut array: Vec<u32>,
    event_tx: mpsc::Sender<SortEvent>,
    sort_rx: &mut mpsc::Receiver<SortMessage>,
) {
    let len = array.len();

    for i in 1..len {
        let key = array[i];
        let mut j = i;

        let _ = event_tx.send(SortEvent::Highlighting(i)).await;
        let _ = event_tx.send(SortEvent::StepIncrement).await;
        tokio::time::sleep(Duration::from_millis(300)).await;

        while j > 0 {
            if let Ok(msg) = sort_rx.try_recv() {
                match msg {
                    SortMessage::Pause => {
                        while let Some(msg) = sort_rx.recv().await {
                            match msg {
                                SortMessage::Continue => break,
                                SortMessage::Stop => return,
                                _ => {}
                            }
                        }
                    }
                    SortMessage::Stop => return,
                    _ => {}
                }
            }

            let _ = event_tx.send(SortEvent::Comparing(j - 1, j)).await;
            let _ = event_tx.send(SortEvent::StepIncrement).await;
            tokio::time::sleep(Duration::from_millis(300)).await;

            if array[j - 1] > key {
                array[j] = array[j - 1];
                let _ = event_tx.send(SortEvent::Swapping(j - 1, j)).await;
                tokio::time::sleep(Duration::from_millis(300)).await;

                j -= 1;
            } else {
                break;
            }
        }

        if array[j] != key {
            array[j] = key;
            let _ = event_tx.send(SortEvent::Highlighting(j)).await;
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    }

    let _ = event_tx.send(SortEvent::Done).await;
}
