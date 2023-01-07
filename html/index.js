window.onload = () => {
    document.getElementById("add_pc-btn").onclick = add_pc;
    document.getElementById("kill-btn").onclick = kill;
}

function add_pc(){
    $.ajax("/get_moonlight/list localhost")
        .done(function(data) {
            alert(data);
        })
        .fail(function(res) {
            alert(res.responseText);
        })
        .always(function() {
            $(this).attr("disabled", false);
        });
}

function kill(){
    $.ajax("/kill/moonlight")
        .done(function(data) {
            alert(data);
        })
        .fail(function(res) {
            alert(res.responseText);
        })
        .always(function() {
            $(this).attr("disabled", false);
        });
    console.log("kill");
}