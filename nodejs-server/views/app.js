$(document).ready(function(){
  $.getJSON("test")
  .then(showEntries)
});

function showEntries(entries){
  entries.forEach(function(entry){
    var newEntry = $('<li class="entry_style">'+ entry.text + '</li>')
    $('.list').append(newEntry)
  })
}
