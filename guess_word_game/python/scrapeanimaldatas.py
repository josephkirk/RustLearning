from requests import get
from requests.exceptions import RequestException
from contextlib import closing
from bs4 import BeautifulSoup
from urllib.parse import urljoin
import asyncio
from pathlib import Path
import logging
import re
from box import Box
import json

logging.basicConfig(level=logging.INFO)

def is_good_response(resp):
    content_type = resp.headers['Content-Type'].lower()
    return all([
        resp.status_code == 200,
        content_type is not None,
        content_type.find('html') > -1
    ])

async def async_simple_get(url):
    try:
        with closing(get(url, stream=True)) as resp:
            if is_good_response(resp):
                return resp.content
            else:
                return
    except RequestException as e:
        logging.error(f'Error durin request to {url} : {e}')


async def get_animal_data(html, data_type):
    finder = html.find(string=re.compile(data_type))
    try:
        return finder.find_parent("td").find_next_sibling().text
    except AttributeError as why:
        logging.error(why)
    return ""

async def get_animal_info(url):
    html = await get_html(url)
    animal_info = Box()
    animal_info.name = html.find("h1").text
    animal_info.type = await get_animal_data(html, "Class")
    animal_info.features = await get_animal_data(html, "Feature")
    logging.info(f"Found {animal_info}")
    return animal_info

async def get_html(url):
    raw_html = await async_simple_get(url)
    return BeautifulSoup(raw_html, "html.parser")

async def main():
    url = 'https://a-z-animals.com'
    html = await get_html(f"{url}/animals/")
    animal_links = (li.a.get('href', '') for li in html.find_all('li', class_=re.compile("az-phobia")))
    tasks = [get_animal_info(urljoin(url,lnk)) for lnk in animal_links]
    results = await asyncio.gather(*tasks)
    filter_results = [animal for animal in results if animal.type]
    # print(filter_results)
    data_file = Path("animal_datas.json")
    if not data_file.exists():
        data_file.touch()
    with data_file.open("r+") as write_file:
        json.dump(filter_results, write_file)

asyncio.run(main())