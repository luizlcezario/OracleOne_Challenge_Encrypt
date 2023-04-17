

function validation(element) {
	if( !element.textContent) {
		element.classList.add("error");
		return false;
	}
	var regex = /[A-Z\u00C0-\u024F]/g;
	var validation = element.textContent.search(regex)
	if(validation != -1) {
		document.getElementById("hit").classList.add("error_hit");
		return false;
	}
	return true;
}

function replaces(text, toRemove, toReplace) {
	var final = "";
	for (var i = 0; i < String(text).length; i++) {
		char = String(text)[i];
		var add = false;
		for (var j = 0; j < toRemove.length; j++) {
			if (toRemove[j].length == 1 && char == toRemove[j]) {
				final += toReplace[j];
				add = true;
			}
			else if (toRemove[j].length > 1 && text.slice(i, i + toRemove[j].length) == toRemove[j]) {
				final += toReplace[j];
				i += toRemove[j].length - 1;
				add = true;
			}
		}
		if (!add) {
			final += char;
			continue;
		}
	}
	return final;
}

function put_res(textContent) {
	document.getElementById("response_text").textContent = textContent;
	document.getElementById("Before").classList.add("hide");
	document.getElementById("Res").classList.add("show");
	document.getElementById("Res").classList.remove("hide");
}

function crypt (){
	var element = document.getElementById("text_content");
	if (!validation(element)) {
		return;
	}
	else {
		var textContent = replaces(element.textContent, ["e", "i", "a", "o", "u"], [ "enter","imes", "ai", "ober", "ufat"])
		put_res(textContent);
	}
}

function deCrypt () {
	var element = document.getElementById("text_content");
	if (!validation(element)) {
		return;
	}
	else {
		var textContent = replaces(element.textContent, [ "enter","imes", "ai", "ober", "ufat"], ["e", "i", "a", "o", "u"])
		put_res(textContent);
	}
}

function copy() {
		var element = document.getElementById("response_text");
		var inputTmp = document.createElement("input");
		inputTmp.value = element.textContent;
		document.body.appendChild(inputTmp);
		inputTmp.select();
		document.execCommand("copy");
		document.body.removeChild(inputTmp);
}