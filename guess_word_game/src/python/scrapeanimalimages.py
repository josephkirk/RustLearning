from google_images_download import google_images_download as goo_id
import json
from pathlib import Path
from PIL import Image
gid = goo_id.googleimagesdownload()

Config = {
    "suffix_keywords": "animal -toy -taxidemy -art -artwork -illustration -jewelry",
    "print_urls": True,
    "type": "photo",
    "format": "jpg",
    "aspect_ratio": "square",
    "limit": 1,
    "size": ">400*300",
    "no_directory": True,
    # "thumbnail": True,
    "safe_search": True,
    "output_directory": "src/image_resources",
}

def download_animal_image(animal_name, limit=1):
    arguments = {
        "keywords": animal_name,
        "limit": limit,
        **Config
    }
    result, _ = gid.download(arguments)
    print(result)
    for image_path in (subresult for result_value in result.values() for subresult in result_value):
        # print(image_path[0])
        print(image_path)
        image_path = Path(image_path)
        new_image_path = image_path.with_name(f"{animal_name}{image_path.suffix}")
        image_path.replace(new_image_path)
        try:
            resizeImage(new_image_path)
        except:
            download_animal_image(resizeImage, limit+1)

def resizeImage(imagepath):
    with Image.open(imagepath) as im:
        resize_im = im.resize((256, 256), Image.LANCZOS)
        resize_im.save(imagepath)

def main():
    animal_datas = json.load(Path("src/animal_datas.json").open())
    animal_names = (animal.get("name") for animal in animal_datas)
    results = map(download_animal_image, animal_names)
    list(results)

if __name__ == "__main__":
    try:
        main()
    except:
        breakpoint