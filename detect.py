import os

import click
import cv2
import imutils
from imutils.perspective import four_point_transform
import numpy as np
from scipy.ndimage.measurements import center_of_mass
from skimage.morphology import remove_small_objects
import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'
tf.get_logger().setLevel('ERROR')


def print_img(im, figsize=(15, 15)):
    # plt.figure(figsize=figsize)
    # plt.imshow(im, cmap='gray')
    cv2.namedWindow("output", cv2.WINDOW_NORMAL)
    cv2.imshow('output', im)
    cv2.waitKey(0)
    cv2.destroyAllWindows()


def crop_img(img, scale=1.0):
    center_x, center_y = img.shape[1] / 2, img.shape[0] / 2
    width_scaled, height_scaled = img.shape[1] * scale, img.shape[0] * scale
    left_x, right_x = center_x - width_scaled / 2, center_x + width_scaled / 2
    top_y, bottom_y = center_y - height_scaled / 2, center_y + height_scaled / 2
    img_cropped = img[int(top_y) : int(bottom_y), int(left_x) : int(right_x)]
    return img_cropped


def find_puzzle(image, debug=False):
    # convert the image to grayscale and blur it slightly
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    blurred = cv2.GaussianBlur(gray, (41, 41), 4)
    # apply adaptive thresholding and then invert the threshold map
    thresh = cv2.adaptiveThreshold(blurred, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 11, 2)
    thresh = cv2.bitwise_not(thresh)
    # check to see if we are visualizing each step of the image
    # processing pipeline (in this case, thresholding)
    if debug:
        print_img(thresh)
    dilated = cv2.dilate(thresh, kernel=np.ones((5, 5), np.uint8), iterations=4)
    if debug:
        print_img(dilated)
    cnts = cv2.findContours(dilated.copy(), cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    cnts = imutils.grab_contours(cnts)
    cnts = sorted(cnts, key=cv2.contourArea, reverse=True)
    # initialize a contour that corresponds to the puzzle outline
    puzzleCnt = []
    # loop over the contours
    for c in cnts:
        # approximate the contour
        peri = cv2.arcLength(c, True)
        approx = cv2.approxPolyDP(c, 0.02 * peri, True)
        # if our approximated contour has four points, then we can
        # assume we have found the outline of the puzzle
        if len(approx) == 4:
            puzzleCnt.append(approx)
            break
        # if the puzzle contour is empty then our script could not find
    # the outline of the Sudoku puzzle so raise an error
    if puzzleCnt is None:
        raise Exception(("Could not find Sudoku puzzle outline. " "Try debugging your thresholding and contour steps."))
    # check to see if we are visualizing the outline of the detected
    # Sudoku puzzle
    if debug:
        # draw the contour of the puzzle on the image and then display
        # it to our screen for visualization/debugging purposes
        output = image.copy()
        cv2.drawContours(output, puzzleCnt, -1, (0, 255, 0), 2)
        print_img(output)
    puzzle = four_point_transform(image, puzzleCnt[0].reshape(4, 2))
    warped = four_point_transform(gray, puzzleCnt[0].reshape(4, 2))
    if debug:
        print_img(warped)

    puzzle = crop_img(puzzle, 0.9)
    warped = crop_img(warped, 0.9)

    if debug:
        print_img(warped)
    return puzzle, warped


def translate_to_com(im, com):
    x_trans = int(im.shape[0] // 2 - com[0])
    y_trans = int(im.shape[1] // 2 - com[1])

    # Pad and remove pixels from image to perform translation

    if x_trans > 0:
        im2 = np.pad(im, ((x_trans, 0), (0, 0)), mode='constant')
        im2 = im2[: im.shape[0] - x_trans, :]
    else:
        im2 = np.pad(im, ((0, -x_trans), (0, 0)), mode='constant')
        im2 = im2[-x_trans:, :]

    if y_trans > 0:
        im3 = np.pad(im2, ((0, 0), (y_trans, 0)), mode='constant')
        im3 = im3[:, : im.shape[0] - y_trans]

    else:
        im3 = np.pad(im2, ((0, 0), (0, -y_trans)), mode='constant')
        im3 = im3[:, -y_trans:]

    return im3


def prepare_digit(cell, cell_w, cell_h, debug):
    sub_img = cell
    if debug:
        print_img(sub_img)
    sub_img = cv2.GaussianBlur(sub_img, (11, 11), 2)
    sub_img = ~(sub_img > 128)
    if debug:
        print_img(sub_img.astype(np.uint8))
    sub_img = remove_small_objects(sub_img, min_size=(cell_w * cell_h * 0.01)) * 255
    if np.sum(sub_img) > 0:
        com = center_of_mass(sub_img)
        sub_img = translate_to_com(sub_img, com)
    if debug:
        print_img(sub_img.astype(np.uint8))
    sub_img = cv2.resize(sub_img.astype(np.uint8), dsize=(28, 28))
    if debug:
        print_img(sub_img)
    sub_img = sub_img.reshape((28, 28, 1))
    return sub_img


def get_not_empty_cells(image, x_size, y_size, debug):
    w, h = image.shape
    cell_w, cell_h = int(w / x_size), int(h / y_size)
    indices = []
    for i in range(x_size):
        for j in range(y_size):
            sub_img = image[cell_w * i : cell_w * (i + 1), cell_h * j : cell_h * (j + 1)]
            cleaned = prepare_digit(sub_img, cell_w, cell_h, debug)
            if np.sum(cleaned) > 0:
                indices.append((i, j))

    return indices


def get_model():
    model = keras.Sequential(
        [
            keras.Input(shape=(28, 28, 1)),
            layers.Conv2D(32, kernel_size=(3, 3), activation="relu"),
            layers.MaxPooling2D(pool_size=(2, 2)),
            layers.Conv2D(64, kernel_size=(3, 3), activation="relu"),
            layers.MaxPooling2D(pool_size=(2, 2)),
            layers.Flatten(),
            layers.Dropout(0.5),
            layers.Dense(4, activation="softmax"),
        ]
    )
    model.compile(loss="categorical_crossentropy", optimizer="adam", metrics=["accuracy"])
    model.load_weights('model.hd5')
    return model


def recognize_digits(warped, non_empty, x_size, y_size, debug):
    model = get_model()
    w, h = warped.shape
    cell_w, cell_h = int(w / x_size), int(h / y_size)
    res = []
    for i, j in non_empty:
        sub_img = warped[cell_w * i : cell_w * (i + 1), cell_h * j : cell_h * (j + 1)]
        sub_img = prepare_digit(sub_img, cell_w, cell_h, False)
        if debug:
            print_img(sub_img / 255.0, figsize=None)
        pred_vector = model.predict(np.array([sub_img > 0]))[0]
        prediction = np.argmax(pred_vector)
        if debug:
            print(f'Prediction {i,j}: {prediction} ({pred_vector[prediction]})')
            print_img(sub_img)
        res.append(prediction)

    return res


@click.command()
@click.option('--img', required=True)
@click.option('--debug', is_flag=True)
@click.option('--x-size', type=int, default=6)
@click.option('--y-size', type=int, default=6)
def detect(img, x_size, y_size, debug):
    im = cv2.imread(img)
    if debug:
        print('debug. initial image:')
        print_img(im)

    puzzle, warped = find_puzzle(im, debug)
    non_empty = get_not_empty_cells(warped, x_size, y_size, False)

    detected_digits = recognize_digits(warped, non_empty, x_size, y_size, debug)
    print(f'Not empty cells at: {non_empty}')
    print(f'Digits are: {detected_digits}')

    result = np.ones((x_size, y_size), dtype=np.int8) * -1

    for (i, j), d in zip(non_empty, detected_digits):
        result[i][j] = d

    print(result)

    for i in range(x_size):
        for j in range(y_size):
            if result[i][j] < 0:
                print(' ', end=' ')
            else:
                print(f'{result[i][j]}', end=' ')
        print()


if __name__ == "__main__":
    detect()
