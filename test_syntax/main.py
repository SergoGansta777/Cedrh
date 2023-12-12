# type: ignore
import smtplib
from email.mime.image import MIMEImage
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from email.mime.base import MIMEBase
from email import encoders

from background import keep_alive 

import telebot

# Create a Telegram bot instance using your API token
bot = telebot.TeleBot("6638422087:AAFEEkz0Mmcv_tfH79Cr5d1bny2Q5znlYqU")

@bot.message_handler(commands=['check'])
def handle_check(message):
    bot.reply_to(message, "Bot is working!")

# Handler for receiving photo and document messages
@bot.message_handler(content_types=["photo", "document"])
def handle_message(message):
    # Check if the message is a photo
    if message.content_type == "photo":
        # Get the photo file id
        file_id = message.photo[-1].file_id
        # Get the file info
        file_info = bot.get_file(file_id)
        # Download the photo file
        file = bot.download_file(file_info.file_path)
        # Send the photo as an email attachment
        send_file(message, file, "photo.jpg", "Фотография от Telegram Bot")
    elif message.content_type == "document":
        # Get the document file id
        file_id = message.document.file_id
        # Get the file info
        file_info = bot.get_file(file_id)
        # Download the document file
        file = bot.download_file(file_info.file_path)
        # Send the document as an email attachment
        send_file(message, file, message.document.file_name, "Файл от Telegram Bot")

# Function to send a file as an email attachment
def send_file(message, file, filename, subject):
    # Create an email message
    msg = MIMEMultipart()
    msg["From"] = "printdocumentstomom@gmail.com"
    msg["To"] = "nikakaanikto0@gmail.com"
    msg["Subject"] = subject

    # Attach the file to the email
    part = MIMEBase('application', "octet-stream")
    part.set_payload(file)
    encoders.encode_base64(part)
    part.add_header('Content-Disposition', 'attachment', filename=filename)  
    msg.attach(part)

    # Send the email using smtplib
    try:
        with smtplib.SMTP("smtp.gmail.com", 587) as smtp:
            smtp.starttls()
            smtp.login("printdocumentstomom@gmail.com", "qetg pchf tozt ohge")
            smtp.send_message(msg)

        friendly_message = (
            f"Успешно отправлено {filename} от кого {msg['From']} кому {msg['To']}"
        )
        print(friendly_message)
        bot.send_message(message.chat.id, friendly_message)
    except smtplib.SMTPException as e:
        friendly_message = f"Не удалось отправить {filename} от кого {msg['From']} кому {msg['To']}: {str(e)}"
        print(friendly_message)
        bot.send_message(message.chat.id, friendly_message)

# Start the bot
keep_alive()
bot.polling()