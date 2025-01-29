export function timeSince (timestamp) {
    let current_date = new Date()
    let previous_date = new Date(timestamp)
    let second_diff = ((current_date.getTime() - previous_date.getTime()) / 1000)
    let message = "a"

    if (second_diff < 60) {
        message = second_diff + " seconds ago";
    } else if (second_diff < 3600) {
        message = Math.round(second_diff / 60) + " minutes ago";
    } else if (second_diff < 86400) {
        message = Math.round(second_diff / 3600) + " hours ago";
    } else if (second_diff < 2620800) {
        message = Math.round(second_diff / 86400) + " days ago";
    } else if (second_diff < 31449600) {
        message = Math.round(second_diff / 2620800) + " months ago";
    } else {
        message = Math.round(second_diff / 31449600) + " years ago";
    }
    return message
}

export function timeToText (timeBefore) {
    if (timeBefore != null) {
        var time = JSON.stringify(timeBefore)
        time = time.replace("-","")

        if (time.length > 4) {
            time = time.slice(0, time.length - 4) + ":" + time.slice(time.length - 4, time.length - 2) + "." + time.slice(time.length - 2)
        } else if (time.length > 2){
            time = time.slice(0, time.length - 2) + "." + time.slice(time.length - 2)
        } else if (time.length === 2){
            time = "0." + time
        } else {
            time = "0.0" + time
        }
    }
    return time
}

export function scoreToText (score) {
    if (score != null) {
        let ms = (score % 100) * 10;
        let seconds = (score / 100) % 60;
        let mins = score / 100 / 60;
        let secondsText = seconds < 10 ? "0" + Math.floor(seconds) : Math.floor(seconds)
        return Math.floor(mins) + ":" + secondsText + "." + Math.round(ms);
    }
}